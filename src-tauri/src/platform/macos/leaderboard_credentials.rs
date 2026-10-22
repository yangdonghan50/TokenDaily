//! Dedicated, non-synchronizing Keychain item owned by TokenDaily.
//! The path scopes the item; no credential or reference file is written on macOS.
use security_framework::passwords::{
    delete_generic_password_options, generic_password, set_generic_password_options,
    PasswordOptions,
};
use sha2::{Digest, Sha256};
use std::path::Path;

const SERVICE: &str = "app.tokendaily.local.leaderboard";
const NOT_FOUND: i32 = -25300;

fn account(path: &Path) -> String {
    format!(
        "{}-{:x}",
        if cfg!(debug_assertions) {
            "development"
        } else {
            "release"
        },
        Sha256::digest(path.as_os_str().as_encoded_bytes())
    )
}
fn options(path: &Path) -> PasswordOptions {
    let mut options = PasswordOptions::new_generic_password(SERVICE, &account(path));
    options.set_access_synchronized(Some(false));
    options
}
fn missing_or_error(code: i32) -> Result<(), &'static str> {
    if code == NOT_FOUND {
        Ok(())
    } else {
        Err("无法访问 TokenDaily 的参榜钥匙串条目，请解锁登录钥匙串并重试；拒绝访问不会被当成未授权")
    }
}
pub(crate) fn load(path: &Path) -> Result<Option<Vec<u8>>, &'static str> {
    match generic_password(options(path)) {
        Ok(bytes) => Ok(Some(bytes)),
        Err(error) => missing_or_error(error.code()).map(|()| None),
    }
}
pub(crate) fn save(path: &Path, bytes: &[u8]) -> Result<(), &'static str> {
    set_generic_password_options(bytes, options(path))
        .map_err(|_| "参榜授权无法保存到 TokenDaily 钥匙串，请检查系统钥匙串权限后重新登录")
}
pub(crate) fn remove(path: &Path) -> Result<(), &'static str> {
    match delete_generic_password_options(options(path)) {
        Ok(()) => Ok(()),
        Err(error) => missing_or_error(error.code()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn denied_is_not_missing_and_paths_are_isolated() {
        assert!(missing_or_error(NOT_FOUND).is_ok());
        assert!(missing_or_error(-25308).is_err());
        assert_ne!(
            account(Path::new("/test/one")),
            account(Path::new("/test/two"))
        );
    }
    #[test]
    #[ignore = "isolated synthetic leaderboard Keychain roundtrip; no network"]
    fn live_leaderboard_keychain_roundtrip() {
        let path = std::env::temp_dir().join(format!(
            "tokendaily-rank-store-{}-{}",
            std::process::id(),
            crate::now_ms()
        ));
        assert!(load(&path).unwrap().is_none());
        save(&path, b"synthetic-first").unwrap();
        let first = load(&path);
        let update = save(&path, b"synthetic-second");
        let second = load(&path);
        remove(&path).unwrap();
        assert_eq!(first.unwrap().unwrap(), b"synthetic-first");
        assert!(update.is_ok());
        assert_eq!(second.unwrap().unwrap(), b"synthetic-second");
        assert!(load(&path).unwrap().is_none());
        assert!(remove(&path).is_ok());
        assert!(!path.exists());
    }
}
