//! WorkBuddy's own credential store; never reads host-client credentials.
#[cfg(target_os = "macos")]
pub(crate) use super::macos::workbuddy_credentials::{load, remove, save};

#[cfg(target_os = "windows")]
pub(crate) fn load(path: &std::path::Path) -> Result<Option<Vec<u8>>, &'static str> {
    match std::fs::read(path) {
        Ok(bytes) => super::unprotect_secret(&bytes).map(Some),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(_) => Err("积分授权读取失败，请检查本机数据目录权限"),
    }
}

#[cfg(target_os = "windows")]
pub(crate) fn save(path: &std::path::Path, bytes: &[u8]) -> Result<(), &'static str> {
    let protected = super::protect_secret(bytes)?;
    std::fs::write(path, protected).map_err(|_| "积分授权安全保存失败")
}

#[cfg(target_os = "windows")]
pub(crate) fn remove(path: &std::path::Path) -> Result<(), &'static str> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(_) => Err("积分授权清除失败，请重试"),
    }
}
