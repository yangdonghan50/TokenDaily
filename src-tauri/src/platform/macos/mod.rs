//! macOS CLI discovery. No shell startup scripts or private bundle paths are used.
pub(crate) mod app_update;
mod claude_credentials;
mod claude_desktop;
pub(crate) mod hotkey;
pub(crate) mod leaderboard_credentials;
pub(crate) mod log_watch;
pub(crate) mod notification;

use std::{
    env,
    ffi::OsStr,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Command,
};

fn search_directories(home: Option<&Path>, path: Option<&OsStr>) -> Vec<PathBuf> {
    let mut paths: Vec<_> = path
        .map(|value| {
            env::split_paths(value)
                .filter(|p| p.is_absolute())
                .collect()
        })
        .unwrap_or_default();
    paths.extend([
        PathBuf::from("/opt/homebrew/bin"),
        PathBuf::from("/usr/local/bin"),
    ]);
    if let Some(home) = home.filter(|p| p.is_absolute()) {
        paths.extend([home.join(".local/bin"), home.join(".npm-global/bin")]);
    }
    paths.extend([
        PathBuf::from("/usr/bin"),
        PathBuf::from("/bin"),
        PathBuf::from("/usr/sbin"),
        PathBuf::from("/sbin"),
    ]);
    let mut unique = Vec::new();
    for path in paths {
        if !unique.contains(&path) {
            unique.push(path);
        }
    }
    unique
}

fn executable_file(path: &Path) -> bool {
    path.is_absolute()
        && path
            .metadata()
            .is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
}

fn resolve_executable(
    explicit: Option<&Path>,
    candidates: Vec<PathBuf>,
) -> Result<PathBuf, &'static str> {
    if let Some(path) = explicit {
        return executable_file(path)
            .then(|| path.to_owned())
            .ok_or("指定的 Codex 程序不存在或不可执行，请在连接设置中重新指定绝对路径");
    }
    candidates
        .into_iter()
        .find(|p| executable_file(p))
        .ok_or("未找到本机 Codex CLI，请在连接设置中指定程序路径")
}

fn directories() -> Vec<PathBuf> {
    search_directories(
        env::var_os("HOME").as_deref().map(Path::new),
        env::var_os("PATH").as_deref(),
    )
}

pub(crate) fn codex_executable(selected: Option<&Path>) -> Result<PathBuf, &'static str> {
    let override_path = env::var_os("TOKENDAILY_CODEX_EXE")
        .filter(|p| !p.is_empty())
        .map(PathBuf::from);
    // A saved explicit choice wins. An invalid explicit choice must never silently
    // fall through to another executable/account.
    resolve_executable(
        selected.or(override_path.as_deref()),
        directories().into_iter().map(|p| p.join("codex")).collect(),
    )
}

pub(crate) fn configure_codex_command(command: &mut Command, executable: &Path) {
    // npm's CLI wrapper may use /usr/bin/env node. Supply the same standard
    // directories only to our child, never change the user's shell configuration.
    let mut paths = directories();
    if let Some(parent) = executable.parent().filter(|p| p.is_absolute()) {
        paths.insert(0, parent.to_owned());
    }
    if let Ok(path) = env::join_paths(paths) {
        command.env("PATH", path);
    }
}

pub(crate) fn codex_home() -> PathBuf {
    env::var_os("CODEX_HOME")
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env::var_os("HOME").unwrap_or_default()).join(".codex"))
}

#[cfg(test)]
mod discovery_tests {
    use super::*;
    use std::{
        fs,
        os::unix::fs::{symlink, PermissionsExt},
        time::{SystemTime, UNIX_EPOCH},
    };

    fn fixture() -> PathBuf {
        let dir = env::temp_dir().join(format!(
            "tokendaily-discovery-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&dir).unwrap();
        dir
    }
    fn executable(path: &std::path::Path) {
        fs::write(path, "#!/bin/sh\nexit 0\n").unwrap();
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
    #[test]
    fn discovery_validates_permissions_and_preserves_explicit_choice() {
        let root = fixture();
        let good = root.join("codex with spaces");
        executable(&good);
        let bad = root.join("not executable");
        fs::write(&bad, "data").unwrap();
        assert_eq!(resolve_executable(Some(&good), vec![]).unwrap(), good);
        assert!(resolve_executable(Some(&bad), vec![good.clone()]).is_err());
        assert!(resolve_executable(Some(&root.join("missing")), vec![good.clone()]).is_err());
        assert_eq!(
            resolve_executable(None, vec![root.clone(), bad, good.clone()]).unwrap(),
            good
        );
        assert!(
            resolve_executable(Some(std::path::Path::new("relative/codex")), vec![good]).is_err()
        );
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn valid_symlinks_work_but_broken_links_do_not() {
        let root = fixture();
        let binary = root.join("native");
        executable(&binary);
        let link = root.join("codex");
        symlink(&binary, &link).unwrap();
        assert!(resolve_executable(None, vec![link.clone()]).is_ok());
        fs::remove_file(binary).unwrap();
        assert!(resolve_executable(None, vec![link]).is_err());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn finder_candidates_include_standard_locations_without_private_bundles() {
        let home = std::path::Path::new("/Users/example");
        let paths = search_directories(Some(home), None);
        assert!(paths.contains(&PathBuf::from("/opt/homebrew/bin")));
        assert!(paths.contains(&PathBuf::from("/usr/local/bin")));
        assert!(paths.contains(&home.join(".local/bin")));
        assert!(paths
            .iter()
            .all(|p| p.is_absolute() && !p.to_string_lossy().contains(".app")));
        let supplied = env::join_paths(["relative", "/custom/bin"]).unwrap();
        assert_eq!(
            search_directories(Some(home), Some(&supplied))[0],
            PathBuf::from("/custom/bin")
        );
    }
}

pub(crate) mod floating;
mod floating_geometry;
pub(crate) mod workbuddy_credentials;
pub(crate) use workbuddy_credentials::open_official_url;

pub(crate) fn claude_credentials() -> Result<String, &'static str> {
    claude_credentials::read()
}

pub(crate) fn claude_desktop_history() -> Result<Vec<u8>, &'static str> {
    claude_desktop::history()
}

pub(crate) fn trae_database() -> PathBuf {
    PathBuf::from(env::var_os("HOME").unwrap_or_default())
        .join("Library/Application Support/Trae CN/ModularData/ai-agent/database.db")
}

pub(crate) fn claude_home() -> PathBuf {
    env::var_os("CLAUDE_CONFIG_DIR")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env::var_os("HOME").unwrap_or_default()).join(".claude"))
}

/// Claude Code's account file: inside CLAUDE_CONFIG_DIR when set, otherwise ~/.claude.json.
pub(crate) fn claude_account_file() -> PathBuf {
    match env::var_os("CLAUDE_CONFIG_DIR").filter(|v| !v.is_empty()) {
        Some(dir) => PathBuf::from(dir).join(".claude.json"),
        None => PathBuf::from(env::var_os("HOME").unwrap_or_default()).join(".claude.json"),
    }
}

pub(crate) fn workbuddy_home() -> std::path::PathBuf {
    std::path::PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".workbuddy")
}
