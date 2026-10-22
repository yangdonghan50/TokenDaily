use crate::app_update::{Snapshot, UpdateState};
use std::{
    sync::Mutex,
    time::{Duration, Instant},
};
use tauri::{Emitter, Manager};
use tauri_plugin_updater::{Update, UpdaterExt};

struct Inner {
    state: UpdateState,
    candidate: Option<Update>,
}
struct Service(Mutex<Inner>);

pub(crate) fn initialize(app: &tauri::AppHandle) -> tauri::Result<()> {
    app.plugin(tauri_plugin_updater::Builder::new().build())?;
    app.manage(Service(Mutex::new(Inner {
        state: UpdateState::new(&app.package_info().version.to_string()),
        candidate: None,
    })));
    let handle = app.clone();
    // A sleeping worker, independent of the 2-second local log collector.
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(5));
        loop {
            let handle = handle.clone();
            tauri::async_runtime::block_on(check(handle, true));
            std::thread::sleep(Duration::from_secs(6 * 3600));
        }
    });
    Ok(())
}

pub(crate) fn snapshot(app: &tauri::AppHandle) -> Snapshot {
    app.state::<Service>()
        .0
        .lock()
        .expect("update state")
        .state
        .snapshot
        .clone()
}
fn emit(app: &tauri::AppHandle) {
    let state = snapshot(app);
    for label in ["main", "floating"] {
        let _ = app.emit_to(label, "app-update", &state);
    }
}

fn allowed_download(update: &Update) -> bool {
    allowed_download_url(&update.version, &update.download_url)
}
fn allowed_download_url(version: &str, url: &reqwest::Url) -> bool {
    url.scheme() == "https"
        && url.host_str() == Some("tokendaily.net")
        && url.port_or_known_default() == Some(443)
        && url.username().is_empty()
        && url.password().is_none()
        && url.query().is_none()
        && url.fragment().is_none()
        && url.path() == format!("/downloads/TokenDaily_{version}_x64-setup.exe")
}

pub(crate) async fn check(app: tauri::AppHandle, automatic: bool) -> Snapshot {
    {
        let service = app.state::<Service>();
        let mut inner = service.0.lock().expect("update state");
        if !inner.state.begin_check(Instant::now(), automatic) {
            return inner.state.snapshot.clone();
        }
    }
    emit(&app);
    let result = async {
        let update = app
            .updater_builder()
            .timeout(Duration::from_secs(20))
            .configure_client(|client| client.redirect(reqwest::redirect::Policy::none()))
            .build()?
            .check()
            .await?;
        Ok::<_, tauri_plugin_updater::Error>(update)
    }
    .await;
    {
        let service = app.state::<Service>();
        let mut inner = service.0.lock().expect("update state");
        match result {
            Ok(update) if update.as_ref().is_none_or(allowed_download) => {
                inner
                    .state
                    .finish_check(update.as_ref().map(|u| u.version.as_str()), None);
                inner.candidate = update;
            }
            Ok(_) => inner
                .state
                .finish_check(None, Some("更新地址不符合官网发布规则，请稍后重试")),
            Err(_) => inner.state.finish_check(
                None,
                Some("暂时无法检查官网更新，请检查网络并于一分钟后重试"),
            ),
        }
    }
    emit(&app);
    snapshot(&app)
}

pub(crate) async fn install(app: tauri::AppHandle) -> Result<Snapshot, String> {
    let mut update = {
        let service = app.state::<Service>();
        let mut inner = service.0.lock().expect("update state");
        let candidate = inner
            .candidate
            .clone()
            .ok_or("暂无可安装的新版本，请先检查更新")?;
        if !inner.state.begin_install() {
            return Ok(inner.state.snapshot.clone());
        }
        candidate
    };
    update.timeout = Some(Duration::from_secs(300));
    emit(&app);
    let mut last_emit = Instant::now();
    let result = update
        .download(
            |chunk, total| {
                {
                    let service = app.state::<Service>();
                    service
                        .0
                        .lock()
                        .expect("update state")
                        .state
                        .progress(chunk, total);
                }
                if last_emit.elapsed() >= Duration::from_millis(250) {
                    emit(&app);
                    last_emit = Instant::now();
                }
            },
            || {},
        )
        .await;
    let result = match result {
        Ok(bytes) => {
            // download verified both the package signature and its signed version.
            app.state::<Service>()
                .0
                .lock()
                .expect("update state")
                .state
                .ready_to_install();
            emit(&app);
            // The plugin launches NSIS with /UPDATE, exiting only after successful
            // launch. NSIS reuses its install path and handles restart (/R).
            update.install(bytes)
        }
        Err(error) => Err(error),
    };
    if result.is_err() {
        app.state::<Service>()
            .0
            .lock()
            .expect("update state")
            .state
            .fail_install("下载、签名校验或安装启动失败，当前版本仍可使用；点击重试");
        emit(&app);
    }
    Ok(snapshot(&app))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn only_exact_official_versioned_downloads_are_accepted() {
        let valid = "https://tokendaily.net/downloads/TokenDaily_0.1.24_x64-setup.exe";
        assert!(allowed_download_url(
            "0.1.24",
            &reqwest::Url::parse(valid).unwrap()
        ));
        for invalid in [
            valid.replace("https:", "http:"),
            valid.replace("tokendaily.net", "tokendaily.net.evil.test"),
            valid.replace("tokendaily.net", "tokendaily.net:444"),
            valid.replace("tokendaily.net", "user@tokendaily.net"),
            format!("{valid}?other=1"),
            format!("{valid}#fragment"),
            valid.replace("0.1.24", "0.1.22"),
            valid.replace("x64-setup.exe", "x64.zip"),
        ] {
            assert!(
                !allowed_download_url("0.1.24", &reqwest::Url::parse(&invalid).unwrap()),
                "{invalid}"
            );
        }
    }
}
