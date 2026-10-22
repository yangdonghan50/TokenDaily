use crate::notification::{Failure, Permission};
use tauri_plugin_notification::NotificationExt;
pub fn install(app: &tauri::AppHandle) -> Result<(), String> {
    let handle = app.clone();
    app.notification()
        .on_action(move |_| crate::show_window(&handle))
        .map_err(|_| "无法注册通知点击回调".into())
}
pub fn permission() -> Result<Permission, &'static str> {
    Ok(Permission::unknown(false))
}
pub fn request_permission() -> Result<Permission, &'static str> {
    Err("当前平台请在系统设置中管理通知权限")
}
pub fn send(
    app: &tauri::AppHandle,
    title: &str,
    body: &str,
    _test: bool,
    still_valid: impl FnOnce() -> bool,
) -> Result<&'static str, Failure> {
    if !still_valid() {
        return Err(Failure::rejected("数据状态已变化，未发送旧提醒"));
    }
    app.notification()
        .builder()
        .title(title)
        .body(body)
        .show()
        .map(|_| "queued")
        .map_err(|_| Failure::rejected("通知组件未接受请求"))
}
pub fn check_test() -> Result<&'static str, &'static str> {
    Ok("unsupported")
}
