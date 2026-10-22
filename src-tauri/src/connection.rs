use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub(crate) struct Settings {
    pub executable: Option<PathBuf>,
    pub home: Option<PathBuf>,
}

#[derive(Clone)]
pub(crate) struct Source {
    pub executable: PathBuf,
    pub home: PathBuf,
}

pub(crate) fn validate_home(path: &Path) -> Result<PathBuf, &'static str> {
    if !path.is_absolute() || !path.is_dir() {
        return Err("Codex 数据目录不存在，请在连接设置中指定已存在目录的绝对路径");
    }
    path.canonicalize().map_err(|_| "无法访问 Codex 数据目录")
}

impl Settings {
    pub(crate) fn home(&self) -> Result<PathBuf, &'static str> {
        let default = crate::platform::codex_home();
        let selected = if cfg!(target_os = "macos") {
            self.home.as_deref().unwrap_or(&default)
        } else {
            &default
        };
        validate_home(selected)
    }
    pub(crate) fn resolve(&self) -> Result<Source, &'static str> {
        Ok(Source {
            executable: crate::platform::codex_executable(self.executable.as_deref())?,
            home: self.home()?,
        })
    }
}

impl Source {
    pub(crate) fn key(&self) -> String {
        let mut digest = Sha256::new();
        for path in [&self.executable, &self.home] {
            let bytes = path.as_os_str().as_encoded_bytes();
            digest.update((bytes.len() as u64).to_le_bytes());
            digest.update(bytes);
        }
        format!("{:x}", digest.finalize())
    }
    pub(crate) fn command(&self) -> Command {
        let mut command = Command::new(&self.executable);
        command.env("CODEX_HOME", &self.home);
        crate::platform::configure_codex_command(&mut command, &self.executable);
        command
    }
}

pub(crate) fn can_change(connected: bool, refreshing: bool) -> Result<(), &'static str> {
    if connected {
        Err("请先断开连接，再修改 Codex 来源")
    } else if refreshing {
        Err("上次读取仍在结束，请稍后再试")
    } else {
        Ok(())
    }
}

pub(crate) fn accept_result(connected: bool, current_epoch: u32, request_epoch: u32) -> bool {
    connected && current_epoch == request_epoch
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Info {
    pub supported: bool,
    pub settings: Settings,
    pub executable: Option<PathBuf>,
    pub home: Option<PathBuf>,
    pub message: Option<String>,
}
impl Info {
    pub(crate) fn new(settings: Settings) -> Self {
        let source = settings.resolve();
        Self {
            supported: cfg!(target_os = "macos"),
            executable: source.as_ref().ok().map(|s| s.executable.clone()),
            home: settings.home().ok(),
            message: source.err().map(str::to_owned),
            settings,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn explicit_home_must_be_absolute_and_exist() {
        assert!(validate_home(std::path::Path::new("relative/home")).is_err());
        let root = std::env::temp_dir();
        assert!(validate_home(&root).is_ok());
        assert!(validate_home(&root.join("tokendaily-deliberately-missing-home")).is_err());
    }
    #[test]
    fn source_fingerprint_separates_executable_and_home() {
        let a = Source {
            executable: "/one/codex".into(),
            home: "/one/home".into(),
        };
        let b = Source {
            executable: "/two/codex".into(),
            home: a.home.clone(),
        };
        let c = Source {
            executable: a.executable.clone(),
            home: "/two/home".into(),
        };
        assert_ne!(a.key(), b.key());
        assert_ne!(a.key(), c.key());
        assert_eq!(a.key(), a.key());
    }
    #[test]
    fn subprocess_uses_selected_home_without_mutating_parent() {
        let before = std::env::var_os("CODEX_HOME");
        let source = Source {
            executable: "/example/codex".into(),
            home: "/selected/codex home".into(),
        };
        let command = source.command();
        assert_eq!(
            command
                .get_envs()
                .find(|(k, _)| *k == "CODEX_HOME")
                .unwrap()
                .1
                .unwrap(),
            source.home.as_os_str()
        );
        assert_eq!(std::env::var_os("CODEX_HOME"), before);
    }
}
