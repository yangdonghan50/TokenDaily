//! Claude Code's own subscription credential. Never enumerate Keychain items or persist a copy.
use security_framework::passwords::get_generic_password;
use std::{fs, io::ErrorKind, path::Path, process::Command};

const SERVICE: &str = "Claude Code-credentials";
const NOT_FOUND: i32 = -25300; // errSecItemNotFound

fn read_with<F>(home: &Path, lookup: F) -> Result<String, &'static str>
where
    F: FnOnce() -> Result<Option<String>, &'static str>,
{
    match lookup()? {
        Some(raw) => Ok(raw),
        None => match fs::read_to_string(home.join(".credentials.json")) {
            Ok(raw) => Ok(raw),
            Err(error) if error.kind() == ErrorKind::NotFound => Err(
                "未找到 Claude Code 订阅登录。请先在 Claude Code 登录；本机日志可独立统计，Claude Desktop 聊天不提供精确 Token",
            ),
            Err(_) => Err("无法读取 Claude Code 凭据文件，请检查文件权限"),
        },
    }
}

pub(crate) fn read() -> Result<String, &'static str> {
    let home = super::claude_home();
    read_with(&home, || {
        // The item account is the OS login name. Do not trust a possibly overridden USER env.
        let output = Command::new("/usr/bin/id")
            .arg("-un")
            .output()
            .map_err(|_| "无法确定本机用户，不能定位 Claude Code 钥匙串凭据")?;
        if !output.status.success() {
            return Err("无法确定本机用户，不能定位 Claude Code 钥匙串凭据");
        }
        let account = std::str::from_utf8(&output.stdout)
            .map(str::trim)
            .ok()
            .filter(|name| !name.is_empty())
            .ok_or("无法确定本机用户，不能定位 Claude Code 钥匙串凭据")?;
        match get_generic_password(SERVICE, &account) {
            Ok(bytes) => String::from_utf8(bytes)
                .map(Some)
                .map_err(|_| "Claude Code 钥匙串凭据格式异常"),
            Err(error) if error.code() == NOT_FOUND => Ok(None),
            Err(_) => Err("Claude Code 钥匙串访问未获允许或读取失败，请在 macOS 系统弹窗中确认"),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn fixture() -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "tokendaily-claude-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&path).unwrap();
        path
    }

    #[test]
    fn keychain_wins_over_file_and_denial_never_uses_stale_file() {
        let home = fixture();
        fs::write(home.join(".credentials.json"), "stale-file").unwrap();
        assert_eq!(
            read_with(&home, || Ok(Some("current-keychain".into()))).unwrap(),
            "current-keychain"
        );
        assert!(read_with(&home, || Err("denied")).is_err());
        assert_eq!(read_with(&home, || Ok(None)).unwrap(), "stale-file");
        fs::remove_dir_all(home).unwrap();
    }

    #[test]
    fn missing_login_is_unknown_not_zero() {
        let home = fixture();
        let error = read_with(&home, || Ok(None)).unwrap_err();
        assert!(error.contains("未找到 Claude Code 订阅登录"));
        fs::remove_dir_all(home).unwrap();
    }
}
