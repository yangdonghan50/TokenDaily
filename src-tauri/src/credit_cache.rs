//! One replaceable last-success balance per provider. No raw responses or credentials.
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{fs, io, path::Path};
#[cfg(target_os = "macos")]
use std::{io::Write, os::unix::fs::OpenOptionsExt};

const MAX_BYTES: usize = 65_536;

#[derive(Serialize, Deserialize)]
struct Record<T> {
    version: u8,
    scope: String,
    balance: T,
}

pub(crate) fn load_last<T: DeserializeOwned>(path: &Path, scope: &str) -> Option<T> {
    let metadata = fs::symlink_metadata(path).ok()?;
    if !metadata.is_file() || metadata.len() > MAX_BYTES as u64 {
        return None;
    }
    let stored = fs::read(path).ok()?;
    if stored.len() > MAX_BYTES {
        return None;
    }
    #[cfg(target_os = "windows")]
    let plain = crate::platform::unprotect_secret(&stored).ok()?;
    // A last balance contains only normalized amounts and a query time. Credentials
    // remain in Keychain; reading another Keychain item at startup would add prompts
    // to every ad-hoc build. Restrict this local cache to the current Unix user.
    #[cfg(target_os = "macos")]
    let plain = {
        use std::os::unix::fs::PermissionsExt;
        if metadata.permissions().mode() & 0o077 != 0 {
            return None;
        }
        stored
    };
    if plain.len() > MAX_BYTES {
        return None;
    }
    let record: Record<T> = serde_json::from_slice(&plain).ok()?;
    (record.version == 1 && record.scope == scope).then_some(record.balance)
}

pub(crate) fn save_last<T: Serialize>(path: &Path, scope: &str, balance: &T) -> Result<(), String> {
    let plain = serde_json::to_vec(&Record {
        version: 1,
        scope: scope.to_owned(),
        balance,
    })
    .map_err(|_| "上次积分序列化失败")?;
    if plain.len() > MAX_BYTES {
        return Err("上次积分记录过大".into());
    }
    #[cfg(target_os = "windows")]
    {
        let encrypted = crate::platform::protect_secret(&plain)?;
        if encrypted.len() > MAX_BYTES {
            return Err("上次积分记录过大".into());
        }
        fs::write(path, encrypted).map_err(|_| "上次积分保存失败".into())
    }
    #[cfg(target_os = "macos")]
    {
        let mut suffix = [0u8; 16];
        getrandom::fill(&mut suffix).map_err(|_| "上次积分保存失败")?;
        let temp = path.with_extension(format!("tmp-{:x}", u128::from_le_bytes(suffix)));
        let result = (|| -> io::Result<()> {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(&temp)?;
            file.write_all(&plain)?;
            file.sync_all()?;
            fs::rename(&temp, path)
        })();
        if result.is_err() {
            let _ = fs::remove_file(&temp);
        }
        result.map_err(|_| "上次积分保存失败".into())
    }
}

pub(crate) fn clear_last(path: &Path) -> io::Result<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::{fs, path::PathBuf};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Example {
        remaining: String,
        collected_at_ms: i64,
    }

    fn path() -> PathBuf {
        let mut bytes = [0u8; 16];
        getrandom::fill(&mut bytes).unwrap();
        std::env::temp_dir().join(format!(
            "tokendaily-last-credit-{:x}",
            u128::from_le_bytes(bytes)
        ))
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn only_last_success_is_stored_and_scope_is_checked() {
        let path = path();
        let first = Example {
            remaining: "12.34".into(),
            collected_at_ms: 1000,
        };
        let latest = Example {
            remaining: "9.87".into(),
            collected_at_ms: 2000,
        };
        save_last(&path, "account-a", &first).unwrap();
        assert_eq!(load_last::<Example>(&path, "account-a"), Some(first));
        assert_eq!(load_last::<Example>(&path, "account-b"), None);
        save_last(&path, "account-a", &latest).unwrap();
        assert_eq!(load_last::<Example>(&path, "account-a"), Some(latest));
        assert!(!String::from_utf8_lossy(&fs::read(&path).unwrap()).contains("9.87"));
        clear_last(&path).unwrap();
        assert_eq!(load_last::<Example>(&path, "account-a"), None);
        clear_last(&path).unwrap();
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn malformed_or_oversized_cache_is_ignored() {
        let path = path();
        fs::write(&path, b"not protected").unwrap();
        assert_eq!(load_last::<Example>(&path, "account-a"), None);
        fs::write(&path, vec![0u8; 65_537]).unwrap();
        assert_eq!(load_last::<Example>(&path, "account-a"), None);
        clear_last(&path).unwrap();
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn mac_cache_is_private_scoped_and_replaceable() {
        use std::os::unix::fs::PermissionsExt;
        let path = path();
        let first = Example {
            remaining: "12.34".into(),
            collected_at_ms: 1000,
        };
        let latest = Example {
            remaining: "9.87".into(),
            collected_at_ms: 2000,
        };
        save_last(&path, "account-a", &first).unwrap();
        assert_eq!(
            fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o600
        );
        assert_eq!(load_last::<Example>(&path, "account-a"), Some(first));
        assert_eq!(load_last::<Example>(&path, "account-b"), None);
        save_last(&path, "account-a", &latest).unwrap();
        assert_eq!(load_last::<Example>(&path, "account-a"), Some(latest));
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
        assert_eq!(load_last::<Example>(&path, "account-a"), None);
        clear_last(&path).unwrap();
    }
}
