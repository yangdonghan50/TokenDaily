//! Native permission and completion callbacks. Never prompt during background refresh.
use crate::{
    alerts::SubmissionError,
    notification::{Failure, Permission},
};
use block2::{DynBlock, RcBlock};
use objc2::{
    define_class, msg_send,
    rc::Retained,
    runtime::{Bool, ProtocolObject},
    AllocAnyThread, DefinedClass,
};
use objc2_foundation::{NSArray, NSBundle, NSError, NSObject, NSObjectProtocol, NSString};
use objc2_user_notifications::*;
use std::{
    cell::RefCell,
    ptr::NonNull,
    sync::{mpsc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const WAIT: Duration = Duration::from_secs(5);
static LAST_TEST: Mutex<Option<(String, bool)>> = Mutex::new(None);
thread_local! { static DELEGATE: RefCell<Option<Retained<TokenDailyNotificationDelegate>>> = const { RefCell::new(None) }; }

define_class!(
    #[unsafe(super(NSObject))]
    #[ivars = tauri::AppHandle]
    struct TokenDailyNotificationDelegate;
    unsafe impl NSObjectProtocol for TokenDailyNotificationDelegate {}
    unsafe impl UNUserNotificationCenterDelegate for TokenDailyNotificationDelegate {
        #[unsafe(method(userNotificationCenter:willPresentNotification:withCompletionHandler:))]
        fn present(
            &self,
            _center: &UNUserNotificationCenter,
            _notification: &UNNotification,
            done: &DynBlock<dyn Fn(UNNotificationPresentationOptions)>,
        ) {
            done.call((
                UNNotificationPresentationOptions::Banner | UNNotificationPresentationOptions::List,
            ));
        }
        #[unsafe(method(userNotificationCenter:didReceiveNotificationResponse:withCompletionHandler:))]
        fn respond(
            &self,
            _center: &UNUserNotificationCenter,
            response: &UNNotificationResponse,
            done: &DynBlock<dyn Fn()>,
        ) {
            // Only a default activation opens the panel; dismiss is not a click.
            if &*response.actionIdentifier() == unsafe { UNNotificationDefaultActionIdentifier } {
                let id = response.notification().request().identifier().to_string();
                if let Ok(mut last) = LAST_TEST.lock() {
                    if let Some((test_id, clicked)) = last.as_mut() {
                        if *test_id == id {
                            *clicked = true;
                        }
                    }
                }
                let app = self.ivars().clone();
                let handle = app.clone();
                let _ = app.run_on_main_thread(move || crate::show_window(&handle));
            }
            done.call(());
        }
    }
);

#[link(name = "Security", kind = "framework")]
unsafe extern "C" {
    fn SecCodeCopySelf(flags: u32, code: *mut *const std::ffi::c_void) -> i32;
    fn SecCodeCheckValidity(
        code: *const std::ffi::c_void,
        flags: u32,
        requirement: *const std::ffi::c_void,
    ) -> i32;
}
#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFRelease(value: *const std::ffi::c_void);
}

fn signature_result(status: i32) -> Result<(), &'static str> {
    match status {
        0 => Ok(()),
        -67062 => Err("当前应用未签名，无法可靠读取或申请系统通知权限；请使用签名后的构建继续验收"),
        _ => Err("无法验证当前应用的通知身份，系统通知权限未知；请检查应用签名"),
    }
}

fn verify_identity() -> Result<(), &'static str> {
    let mut code = std::ptr::null();
    // Security owns the retained reference returned by CopySelf; release once.
    // Default flags and no extra requirement validate this running process only.
    let status = unsafe { SecCodeCopySelf(0, &mut code) };
    signature_result(status)?;
    if code.is_null() {
        return signature_result(-1);
    }
    let status = unsafe { SecCodeCheckValidity(code, 0, std::ptr::null()) };
    unsafe { CFRelease(code) };
    signature_result(status)
}

fn center() -> Result<Retained<UNUserNotificationCenter>, &'static str> {
    let bundle = NSBundle::mainBundle();
    // UNUserNotificationCenter can throw for an unbundled CLI/test binary.
    if bundle
        .bundleIdentifier()
        .as_deref()
        .map(NSString::to_string)
        .as_deref()
        != Some("app.tokendaily.local")
        || !bundle.bundlePath().to_string().ends_with(".app")
    {
        return Err("请打开构建后的 TokenDaily.app 再检查系统通知");
    }
    // An unsigned process can receive default settings even though the service
    // rejected its query. Do not report those defaults as an observed permission.
    verify_identity()?;
    Ok(UNUserNotificationCenter::currentNotificationCenter())
}

pub(crate) fn install(app: &tauri::AppHandle) -> Result<(), String> {
    let Ok(center) = center() else {
        return Ok(());
    };
    let allocated = TokenDailyNotificationDelegate::alloc().set_ivars(app.clone());
    // NSObject init establishes the object; ivars contain only a thread-safe AppHandle.
    let delegate: Retained<TokenDailyNotificationDelegate> =
        unsafe { msg_send![super(allocated), init] };
    center.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));
    DELEGATE.with(|slot| *slot.borrow_mut() = Some(delegate)); // center's delegate is weak.
    Ok(())
}

fn normalize(status: isize, alerts: isize, center: isize) -> Permission {
    let setting = |value| match value {
        1 => Some(false),
        2 => Some(true),
        _ => None,
    };
    Permission {
        supported: true,
        authorization: match status {
            0 => "not-determined",
            1 => "denied",
            2 => "authorized",
            3 => "provisional",
            _ => "unknown",
        },
        alerts: setting(alerts),
        center: setting(center),
    }
}

pub(crate) fn permission() -> Result<Permission, &'static str> {
    let center = center()?;
    let (tx, rx) = mpsc::sync_channel(1);
    let callback = RcBlock::new(move |settings: NonNull<UNNotificationSettings>| {
        // Valid for the duration of the framework's callback; no pointer escapes.
        let settings = unsafe { settings.as_ref() };
        let _ = tx.try_send(normalize(
            settings.authorizationStatus().0,
            settings.alertSetting().0,
            settings.notificationCenterSetting().0,
        ));
    });
    center.getNotificationSettingsWithCompletionHandler(&callback);
    rx.recv_timeout(WAIT)
        .map_err(|_| "读取系统通知权限超时，权限仍未知")
}

pub(crate) fn request_permission() -> Result<Permission, &'static str> {
    let current = permission()?;
    if current.authorization != "not-determined" {
        return Ok(current);
    }
    let (tx, rx) = mpsc::sync_channel(1);
    let callback = RcBlock::new(move |_granted: Bool, error: *mut NSError| {
        let _ = tx.try_send(error.is_null());
    });
    // Alert-only: no badges, sound, provisional grant or interruption-level escalation.
    center()?.requestAuthorizationWithOptions_completionHandler(
        UNAuthorizationOptions::Alert,
        &callback,
    );
    match rx.recv_timeout(Duration::from_secs(60)) {
        Ok(true) => permission(),
        Ok(false) => Err("系统未能完成通知授权请求，请检查系统设置"),
        Err(_) => Err("尚未收到系统授权结果，请完成系统提示后刷新权限状态"),
    }
}

pub(crate) fn send(
    _app: &tauri::AppHandle,
    title: &str,
    body: &str,
    test: bool,
    still_valid: impl FnOnce() -> bool,
) -> Result<&'static str, Failure> {
    let permission = permission().map_err(Failure::rejected)?;
    if !permission.can_submit() {
        return Err(Failure::rejected(
            "系统通知尚未允许或显示通道已关闭，请检查通知权限",
        ));
    }
    let center = center().map_err(Failure::rejected)?;
    if !still_valid() {
        return Err(Failure::rejected("数据状态已变化，未发送旧提醒"));
    }
    let id = format!(
        "tokendaily-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    );
    if test {
        *LAST_TEST.lock().expect("test notification lock") = Some((id.clone(), false));
    }
    let content = UNMutableNotificationContent::new();
    content.setTitle(&NSString::from_str(title));
    content.setBody(&NSString::from_str(body));
    let request = UNNotificationRequest::requestWithIdentifier_content_trigger(
        &NSString::from_str(&id),
        &content,
        None,
    );
    let (tx, rx) = mpsc::sync_channel(1);
    let callback = RcBlock::new(move |error: *mut NSError| {
        let _ = tx.try_send(error.is_null());
    });
    center.addNotificationRequest_withCompletionHandler(&request, Some(&callback));
    submission_result(rx.recv_timeout(WAIT))
}

fn submission_result(
    result: Result<bool, mpsc::RecvTimeoutError>,
) -> Result<&'static str, Failure> {
    match result {
        Ok(true) => Ok("accepted"),
        Ok(false) => Err(Failure::rejected("系统拒绝了通知请求，请检查通知设置")),
        Err(_) => Err(Failure {
            kind: SubmissionError::Uncertain,
            message: "通知提交结果未知，请先检查通知中心；为避免重复不会自动重发",
        }),
    }
}

pub(crate) fn check_test() -> Result<&'static str, &'static str> {
    let Some((id, clicked)) = LAST_TEST.lock().expect("test notification lock").clone() else {
        return Ok("none");
    };
    if clicked {
        return Ok("clicked");
    }
    let (tx, rx) = mpsc::sync_channel(1);
    let callback = RcBlock::new(move |items: NonNull<NSArray<UNNotification>>| {
        let found = unsafe { items.as_ref() }
            .iter()
            .any(|item| item.request().identifier().to_string() == id);
        let _ = tx.try_send(found);
    });
    center()?.getDeliveredNotificationsWithCompletionHandler(&callback);
    rx.recv_timeout(WAIT)
        .map(|found| if found { "in-center" } else { "not-observed" })
        .map_err(|_| "读取测试通知状态超时，不能确认是否展示")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn only_a_callback_rejection_is_retryable_not_a_missing_callback() {
        assert_eq!(submission_result(Ok(true)).unwrap(), "accepted");
        assert_eq!(
            submission_result(Ok(false)).unwrap_err().kind,
            SubmissionError::Rejected
        );
        for error in [
            mpsc::RecvTimeoutError::Timeout,
            mpsc::RecvTimeoutError::Disconnected,
        ] {
            assert_eq!(
                submission_result(Err(error)).unwrap_err().kind,
                SubmissionError::Uncertain
            );
        }
    }
    #[test]
    fn signature_failure_never_becomes_permission_not_determined() {
        assert!(signature_result(0).is_ok());
        assert!(signature_result(-67062).unwrap_err().contains("未签名"));
        assert!(signature_result(-67050).is_err());
        assert!(signature_result(-1).is_err());
    }
    #[test]
    fn normalizes_actual_os_values_without_treating_unknown_as_allowed() {
        assert_eq!(normalize(0, 0, 0).authorization, "not-determined");
        assert!(!normalize(1, 2, 2).can_submit());
        assert!(normalize(2, 2, 1).can_submit());
        assert!(normalize(3, 1, 2).can_submit());
        assert!(!normalize(99, 2, 2).can_submit());
        assert!(!normalize(2, 1, 1).can_submit());
        assert_eq!(normalize(2, 99, 0).alerts, None);
    }
    #[test]
    fn unbundled_tests_report_unavailable_without_calling_notification_center() {
        assert!(center().is_err());
    }
}
