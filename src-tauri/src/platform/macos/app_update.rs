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
    allowed_download_url(
        &update.version,
        std::env::consts::ARCH,
        &update.download_url,
    )
}
/// Bundle names this running architecture may install. Apple silicon never accepts
/// an Intel-only bundle, which would silently move the install onto Rosetta.
fn bundle_names(arch: &str) -> &'static [&'static str] {
    match arch {
        "x86_64" => &["x64", "universal"],
        "aarch64" => &["universal"],
        _ => &[],
    }
}
fn allowed_download_url(version: &str, arch: &str, url: &reqwest::Url) -> bool {
    url.scheme() == "https"
        && url.host_str() == Some("tokendaily.net")
        && url.port_or_known_default() == Some(443)
        && url.username().is_empty()
        && url.password().is_none()
        && url.query().is_none()
        && url.fragment().is_none()
        && bundle_names(arch)
            .iter()
            .any(|name| url.path() == format!("/downloads/TokenDaily_{version}_{name}.app.tar.gz"))
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
            Err(tauri_plugin_updater::Error::ReleaseNotFound) => inner
                .state
                .finish_check(None, Some("Mac 更新清单尚未发布或暂时不可访问，请稍后重试")),
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
            // The verified macOS bundle replaces the installed app. Restart only
            // after the plugin reports a successful installation.
            update.install(bytes)
        }
        Err(error) => Err(error),
    };
    if result.is_ok() {
        app.restart();
    } else {
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
    fn url(s: &str) -> reqwest::Url {
        reqwest::Url::parse(s).unwrap()
    }
    const BASE: &str = "https://tokendaily.net/downloads/TokenDaily_0.1.28";

    #[test]
    fn intel_accepts_its_own_or_the_universal_bundle() {
        for name in ["x64", "universal"] {
            let valid = url(&format!("{BASE}_{name}.app.tar.gz"));
            assert!(allowed_download_url("0.1.28", "x86_64", &valid), "{name}");
        }
        assert!(!allowed_download_url(
            "0.1.28",
            "x86_64",
            &url(&format!("{BASE}_aarch64.app.tar.gz"))
        ));
    }

    #[test]
    fn apple_silicon_only_accepts_the_universal_bundle() {
        assert!(allowed_download_url(
            "0.1.28",
            "aarch64",
            &url(&format!("{BASE}_universal.app.tar.gz"))
        ));
        // An Intel-only bundle would downgrade an Apple silicon install to Rosetta.
        for name in ["x64", "aarch64"] {
            assert!(
                !allowed_download_url(
                    "0.1.28",
                    "aarch64",
                    &url(&format!("{BASE}_{name}.app.tar.gz"))
                ),
                "{name}"
            );
        }
        assert!(!allowed_download_url(
            "0.1.28",
            "riscv64",
            &url(&format!("{BASE}_universal.app.tar.gz"))
        ));
    }

    #[test]
    fn only_exact_official_mac_bundle_is_accepted() {
        let valid = format!("{BASE}_universal.app.tar.gz");
        for arch in ["x86_64", "aarch64"] {
            assert!(allowed_download_url("0.1.28", arch, &url(&valid)));
            for invalid in [
                valid.replace("https:", "http:"),
                valid.replace("tokendaily.net", "tokendaily.net.evil.test"),
                valid.replace("tokendaily.net", "tokendaily.net:444"),
                valid.replace("tokendaily.net", "user@tokendaily.net"),
                format!("{valid}?other=1"),
                format!("{valid}#fragment"),
                valid.replace("0.1.28", "0.1.27"),
                valid.replace("universal.app.tar.gz", "universal-setup.exe"),
                valid.replace("/downloads/", "/downloads/x/"),
            ] {
                assert!(
                    !allowed_download_url("0.1.28", arch, &url(&invalid)),
                    "{arch} {invalid}"
                );
            }
        }
    }

    #[test]
    fn endpoint_is_chosen_by_the_running_architecture() {
        let config: serde_json::Value =
            serde_json::from_str(include_str!("../../../tauri.macos.conf.json")).unwrap();
        let endpoints = config["plugins"]["updater"]["endpoints"]
            .as_array()
            .unwrap();
        assert_eq!(endpoints.len(), 1);
        // Intel keeps requesting the existing darwin-x86_64.json manifest.
        assert_eq!(
            endpoints[0],
            "https://tokendaily.net/updates/darwin-{{arch}}.json"
        );
    }
}
