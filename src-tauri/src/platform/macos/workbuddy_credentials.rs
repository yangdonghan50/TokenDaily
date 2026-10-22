//! Dedicated, non-synchronizing Keychain item owned by TokenDaily.
//! The path scopes the item; no credential or reference file is written on macOS.
use security_framework::passwords::{
    delete_generic_password_options, generic_password, set_generic_password_options,
    PasswordOptions,
};
use sha2::{Digest, Sha256};
use std::{
    path::Path,
    process::{Command, Stdio},
};

const SERVICE: &str = "app.tokendaily.local.workbuddy-credits";
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
        Err("无法访问 TokenDaily 的积分钥匙串条目，请解锁登录钥匙串并重试；拒绝访问不会被当成未授权")
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
        .map_err(|_| "积分授权无法保存到 TokenDaily 钥匙串，请检查系统钥匙串权限后重新登录")
}
pub(crate) fn remove(path: &Path) -> Result<(), &'static str> {
    match delete_generic_password_options(options(path)) {
        Ok(()) => Ok(()),
        Err(error) => missing_or_error(error.code()),
    }
}
fn official_url(url: &str) -> bool {
    reqwest::Url::parse(url).is_ok_and(|u| {
        let public_participant = u.fragment().is_some_and(|fragment| {
            let Some(id) = fragment.strip_prefix("participant=") else {
                return false;
            };
            id.len() == 36
                && id.bytes().enumerate().all(|(index, byte)| {
                    if [8, 13, 18, 23].contains(&index) {
                        byte == b'-'
                    } else {
                        byte.is_ascii_hexdigit()
                    }
                })
        });
        let pairing_code = u.fragment().is_some_and(|fragment| {
            fragment.strip_prefix("pair=").is_some_and(|code| {
                code.len() == 48 && code.bytes().all(|byte| byte.is_ascii_hexdigit())
            })
        });
        u.scheme() == "https"
            && u.username().is_empty()
            && u.password().is_none()
            && u.port_or_known_default() == Some(443)
            && (u.host_str() == Some("tokendaily.net")
                && u.query().is_none()
                && ((u.path() == "/leaderboard/"
                    && (u.fragment().is_none() || public_participant))
                    || (u.path() == "/zh/account/" && (u.fragment().is_none() || pairing_code)))
                || matches!(
                    u.host_str(),
                    Some("codebuddy.cn" | "www.codebuddy.cn" | "workbuddy.cn" | "www.workbuddy.cn")
                ))
    })
}
pub(crate) fn open_official_url(url: &str) -> Result<(), &'static str> {
    if !official_url(url) {
        return Err("仅允许打开指定官方 HTTPS 登录页面");
    }
    let status = Command::new("/usr/bin/open")
        .arg(url)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|_| "无法打开默认浏览器，请检查系统设置")?;
    if status.success() {
        Ok(())
    } else {
        Err("无法打开默认浏览器，请检查系统设置")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn own_store_is_scoped_and_denial_is_not_missing() {
        assert_ne!(account(Path::new("/test/a")), account(Path::new("/test/b")));
        assert_eq!(account(Path::new("/test/a")), account(Path::new("/test/a")));
        assert!(missing_or_error(NOT_FOUND).is_ok());
        for code in [-25293, -25308, -128] {
            assert!(missing_or_error(code).is_err());
        }
    }
    #[test]
    fn browser_launch_accepts_only_official_https_urls() {
        assert!(official_url(
            "https://www.codebuddy.cn/login?state=synthetic"
        ));
        assert!(official_url("https://www.workbuddy.cn/login"));
        assert!(official_url("https://tokendaily.net/leaderboard/"));
        assert!(official_url(
            "https://tokendaily.net/leaderboard/#participant=12345678-1234-1234-1234-123456789abc"
        ));
        assert!(official_url("https://tokendaily.net/zh/account/"));
        assert!(official_url(&format!(
            "https://tokendaily.net/zh/account/#pair={}",
            "a".repeat(48)
        )));
        assert!(!official_url("https://tokendaily.net/other"));
        for url in [
            "file:///tmp/a",
            "http://www.codebuddy.cn/",
            "https://evil.test/",
            "https://www.codebuddy.cn.evil.test/",
            "https://user@www.codebuddy.cn/",
            "https://www.codebuddy.cn:444/",
            "https://tokendaily.net/leaderboard/#participant=secret-token",
            "https://tokendaily.net/leaderboard/?access_token=secret#participant=12345678-1234-1234-1234-123456789abc",
            "https://tokendaily.net/zh/account/#pair=short",
        ] {
            assert!(!official_url(url));
        }
    }
    #[test]
    #[ignore = "writes and removes an isolated synthetic TokenDaily Keychain item"]
    fn live_workbuddy_keychain_roundtrip() {
        let path = std::env::temp_dir().join(format!(
            "tokendaily-keychain-test-{}-{}",
            std::process::id(),
            crate::now_ms()
        ));
        assert!(load(&path).unwrap().is_none());
        save(&path, b"synthetic-first").unwrap();
        let first = load(&path);
        let update = save(&path, b"synthetic-second");
        let second = load(&path);
        let deleted = remove(&path);
        assert!(deleted.is_ok());
        assert_eq!(first.unwrap().unwrap(), b"synthetic-first");
        assert!(update.is_ok());
        assert_eq!(second.unwrap().unwrap(), b"synthetic-second");
        assert!(load(&path).unwrap().is_none());
        assert!(remove(&path).is_ok());
        assert!(!path.exists());
    }
}
