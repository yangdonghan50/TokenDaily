//! Reads Claude Desktop's own plan usage history file. No browser session or
//! credential access; parsing and product rules live in the shared `claude` module.

use std::{
    env, fs,
    io::{ErrorKind, Read},
    path::Path,
};

// Claude Desktop retains roughly a month of samples, so a normal long-lived
// file can be much larger than a single response payload.
const MAX_FILE_BYTES: u64 = 4_194_304;

pub(crate) fn history() -> Result<Vec<u8>, &'static str> {
    let home = env::var_os("HOME")
        .filter(|s| !s.is_empty())
        .ok_or("无法定位 Claude Desktop 本机数据目录")?;
    let path = Path::new(&home).join("Library/Application Support/Claude/plan-usage-history.json");
    let metadata = fs::symlink_metadata(&path).map_err(|error| match error.kind() {
        ErrorKind::NotFound => {
            "未找到 Claude Desktop 本机额度历史；打开 Claude Desktop 的用量页后再刷新"
        }
        ErrorKind::PermissionDenied => {
            "TokenDaily 无法访问 Claude Desktop 本机额度历史；请检查 macOS 文件访问权限"
        }
        _ => "无法检查 Claude Desktop 本机额度历史",
    })?;
    if !metadata.file_type().is_file() || metadata.len() > MAX_FILE_BYTES {
        return Err("Claude Desktop 本机额度历史文件无效或过大");
    }
    let mut bytes = Vec::new();
    fs::File::open(path)
        .map_err(|error| match error.kind() {
            ErrorKind::PermissionDenied => {
                "TokenDaily 无法读取 Claude Desktop 本机额度历史；请检查 macOS 文件访问权限"
            }
            _ => "无法打开 Claude Desktop 本机额度历史",
        })?
        .take(MAX_FILE_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "无法读取 Claude Desktop 本机额度历史")?;
    if bytes.len() as u64 > MAX_FILE_BYTES {
        return Err("Claude Desktop 本机额度历史文件无效或过大");
    }
    Ok(bytes)
}
