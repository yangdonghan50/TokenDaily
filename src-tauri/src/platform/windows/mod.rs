pub(crate) mod app_update;
pub(crate) mod floating_drag;
pub(crate) mod hotkey;
pub(crate) mod log_watch;
pub(crate) mod notification;
mod secrets;
pub(crate) use secrets::{open_official_url, protect_secret, unprotect_secret};

use std::{env, fs, path::PathBuf};

pub(crate) fn trae_database() -> PathBuf {
    PathBuf::from(env::var_os("APPDATA").unwrap_or_default())
        .join("Trae CN/ModularData/ai-agent/database.db")
}

pub(crate) fn workbuddy_home() -> PathBuf {
    PathBuf::from(env::var_os("USERPROFILE").unwrap_or_default()).join(".workbuddy")
}

pub(crate) fn claude_credentials() -> Result<String, &'static str> {
    fs::read_to_string(claude_home().join(".credentials.json")).map_err(|_| {
        "未发现 Claude Code 订阅登录；请在 Claude Desktop 中打开设置的用量页以生成本机额度记录"
    })
}

pub(crate) fn claude_desktop_history() -> Result<Vec<u8>, &'static str> {
    let appdata = env::var_os("APPDATA").ok_or("无法定位 Claude Desktop 用户数据目录")?;
    let path = PathBuf::from(appdata).join("Claude/plan-usage-history.json");
    let file = fs::File::open(path)
        .map_err(|_| "Claude Desktop 尚无本机额度记录；请打开设置中的用量页")?;
    let mut bytes = Vec::new();
    use std::io::Read;
    file.take(262_145)
        .read_to_end(&mut bytes)
        .map_err(|_| "Claude Desktop 额度记录读取失败")?;
    if bytes.len() > 262_144 {
        return Err("Claude Desktop 额度记录过大");
    }
    Ok(bytes)
}

pub(crate) fn claude_account_file() -> PathBuf {
    match env::var_os("CLAUDE_CONFIG_DIR").filter(|v| !v.is_empty()) {
        Some(dir) => PathBuf::from(dir).join(".claude.json"),
        None => PathBuf::from(env::var_os("USERPROFILE").unwrap_or_default()).join(".claude.json"),
    }
}

pub(crate) fn claude_home() -> PathBuf {
    env::var_os("CLAUDE_CONFIG_DIR")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env::var_os("USERPROFILE").unwrap_or_default()).join(".claude")
        })
}

pub(crate) fn codex_executable() -> Option<PathBuf> {
    if let Some(path) = env::var_os("TOKENDAILY_CODEX_EXE") {
        let path = PathBuf::from(path);
        if path.is_file() {
            return Some(path);
        }
    }
    if let Some(paths) = env::var_os("PATH") {
        for dir in env::split_paths(&paths) {
            let candidate = dir.join("codex.exe");
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    // Codex Desktop keeps versioned CLI binaries here. Inspect only this known directory.
    let local = PathBuf::from(env::var_os("LOCALAPPDATA")?)
        .join("OpenAI")
        .join("Codex")
        .join("bin");
    let mut candidates: Vec<_> = fs::read_dir(local)
        .ok()?
        .flatten()
        .map(|entry| entry.path().join("codex.exe"))
        .filter(|path| path.is_file())
        .collect();
    candidates.sort_by_key(|path| fs::metadata(path).and_then(|meta| meta.modified()).ok());
    candidates.pop()
}

pub(crate) fn codex_home() -> PathBuf {
    env::var_os("CODEX_HOME")
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env::var_os("USERPROFILE").unwrap_or_default()).join(".codex")
        })
}
