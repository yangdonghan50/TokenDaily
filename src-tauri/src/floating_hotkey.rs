//! Own hotkey preference only; no key stream or typed text is collected.
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::Mutex,
};
use tauri::{Emitter, Manager};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Shortcut {
    pub key: u32,
    pub modifiers: u32,
}
impl Shortcut {
    fn validate(self) -> Result<Self, String> {
        let key = self.key;
        if self.modifiers > 15
            || !matches!(key, 8 | 9 | 13 | 27 | 32..=40 | 45..=46 | 48..=57 | 65..=90 | 96..=107 | 109..=122 | 124..=135 | 186..=192 | 219..=222)
        {
            return Err("该按键不受支持；修饰键需搭配普通键，F12 为系统保留键".into());
        }
        Ok(self)
    }
}
pub(crate) trait Registry: Send {
    fn replace(&mut self, value: Option<Shortcut>) -> Result<(), String>;
}

#[derive(Serialize, Deserialize)]
struct Saved {
    generation: u64,
    shortcut: Option<Shortcut>,
}
fn slots(path: &Path) -> [PathBuf; 2] {
    [path.with_extension("a.json"), path.with_extension("b.json")]
}
fn read_saved(path: &Path) -> Saved {
    slots(path)
        .iter()
        .filter_map(|slot| {
            let value: Saved = serde_json::from_slice(&fs::read(slot).ok()?).ok()?;
            if value.shortcut.is_some_and(|key| key.validate().is_err()) {
                return None;
            }
            Some(value)
        })
        .max_by_key(|value| value.generation)
        .unwrap_or(Saved {
            generation: 0,
            shortcut: None,
        })
}
fn persist(path: &Path, shortcut: Option<Shortcut>) -> Result<(), String> {
    let generation = read_saved(path)
        .generation
        .checked_add(1)
        .ok_or("快捷键配置版本无效")?;
    let bytes = serde_json::to_vec(&Saved {
        generation,
        shortcut,
    })
    .map_err(|_| "快捷键编码失败")?;
    // Keep the last complete value while writing the other slot. This also
    // supports redirected Windows folders that cannot replace files by rename.
    let destination = &slots(path)[generation as usize % 2];
    let mut file =
        fs::File::create(destination).map_err(|_| "快捷键保存失败，请检查目录权限和磁盘空间")?;
    file.write_all(&bytes)
        .and_then(|_| file.sync_all())
        .map_err(|_| "快捷键保存失败，请检查目录权限和磁盘空间".to_string())
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Status {
    shortcut: Option<Shortcut>,
    supported: bool,
    active: bool,
    recording: bool,
    error: Option<String>,
}
struct Inner {
    registry: Box<dyn Registry>,
    status: Status,
    enabled: bool,
    generation: u64,
}
impl Inner {
    fn apply(&mut self) {
        let requested = if self.enabled && !self.status.recording {
            self.status.shortcut
        } else {
            None
        };
        match self.registry.replace(requested) {
            Ok(()) => {
                self.status.active = requested.is_some();
                self.status.error = None;
            }
            Err(error) => {
                self.status.error = Some(error);
                self.status.active = false;
            }
        }
    }
    fn save(
        &mut self,
        value: Option<Shortcut>,
        persist: impl FnOnce() -> Result<(), String>,
    ) -> Result<(), String> {
        if let Some(key) = value {
            key.validate()?;
        }
        let target = if self.enabled { value } else { None };
        // Registration tests conflict even while recording. A failed registration
        // never replaces the saved preference; resume the original binding.
        if let Err(error) = self.registry.replace(target) {
            self.status.recording = false;
            self.generation += 1;
            self.apply();
            return Err(error);
        }
        if let Err(error) = persist() {
            self.status.recording = false;
            self.generation += 1;
            self.apply();
            return Err(error);
        }
        self.status.shortcut = value;
        self.status.active = target.is_some();
        self.status.recording = false;
        self.status.error = None;
        self.generation += 1;
        Ok(())
    }
}
pub(crate) struct Service {
    inner: Mutex<Inner>,
    path: PathBuf,
}
struct Unsupported;
impl Registry for Unsupported {
    fn replace(&mut self, value: Option<Shortcut>) -> Result<(), String> {
        if value.is_some() {
            Err("当前平台尚不支持悬浮窗快捷键".into())
        } else {
            Ok(())
        }
    }
}
impl Service {
    pub(crate) fn new(app: tauri::AppHandle, directory: &std::path::Path, enabled: bool) -> Self {
        let path = directory.join("floating-hotkey.json");
        let saved = read_saved(&path).shortcut;
        let event_app = app.clone();
        let toggle = move || {
            if event_app
                .state::<crate::AppState>()
                .floating_enabled
                .load(std::sync::atomic::Ordering::SeqCst)
            {
                let _ = event_app.emit_to("floating", "floating-shortcut-toggle", ());
            }
        };
        #[cfg(target_os = "windows")]
        let driver = crate::platform::hotkey::Driver::new(toggle);
        #[cfg(target_os = "macos")]
        let driver = crate::platform::hotkey::Driver::new(app, toggle);
        let driver = driver.map(|driver| Box::new(driver) as Box<dyn Registry>);
        let (registry, error) = match driver {
            Ok(driver) => (driver, None),
            Err(error) => (Box::new(Unsupported) as Box<dyn Registry>, Some(error)),
        };
        let supported = error.is_none();
        let mut inner = Inner {
            registry,
            status: Status {
                shortcut: saved,
                supported,
                active: false,
                recording: false,
                error,
            },
            enabled,
            generation: 0,
        };
        if supported {
            inner.apply();
        }
        Self {
            inner: Mutex::new(inner),
            path,
        }
    }
    pub(crate) fn enabled(&self, enabled: bool) {
        let mut inner = self.inner.lock().unwrap();
        inner.enabled = enabled;
        if !enabled {
            inner.status.recording = false;
            inner.generation += 1;
        }
        if inner.status.supported {
            inner.apply();
        }
    }
    fn status(&self) -> Status {
        self.inner.lock().unwrap().status.clone()
    }
    fn save(&self, shortcut: Option<Shortcut>) -> Result<(), String> {
        let mut inner = self.inner.lock().unwrap();
        if !inner.status.supported {
            return Err("当前平台尚不支持悬浮窗快捷键".into());
        }
        inner.save(shortcut, || persist(&self.path, shortcut))
    }
    pub(crate) fn clear(&self) -> Result<(), String> {
        let mut inner = self.inner.lock().unwrap();
        // Persist a cleared value first so a later cleanup error cannot revive
        // an old binding at next startup.
        inner.save(None, || persist(&self.path, None))?;
        let latest = read_saved(&self.path).generation as usize % 2;
        let paths = slots(&self.path);
        for path in [&paths[1 - latest], &paths[latest]] {
            match fs::remove_file(path) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(_) => return Err("清除快捷键失败".into()),
            }
        }
        Ok(())
    }
    fn record(&self, recording: bool) -> Result<u64, String> {
        let mut inner = self.inner.lock().unwrap();
        if !inner.status.supported {
            return Err("当前平台尚不支持悬浮窗快捷键".into());
        }
        if recording {
            inner.registry.replace(None)?;
        }
        inner.generation += 1;
        inner.status.recording = recording;
        inner.apply();
        Ok(inner.generation)
    }
    fn expire(&self, generation: u64) {
        let mut inner = self.inner.lock().unwrap();
        if inner.generation == generation && inner.status.recording {
            inner.status.recording = false;
            inner.apply();
        }
    }
}
pub(crate) fn publish(app: &tauri::AppHandle) {
    let _ = app.emit("floating-hotkey-updated", app.state::<Service>().status());
}
// On Mac the service lock and Carbon operations must run on the same thread;
// otherwise a synchronous status read can deadlock a worker waiting for AppKit.
pub(crate) fn on_thread<T: Send + 'static>(
    app: &tauri::AppHandle,
    task: impl FnOnce() -> Result<T, String> + Send + 'static,
) -> Result<T, String> {
    #[cfg(target_os = "macos")]
    {
        crate::platform::hotkey::on_main(app, task)
    }
    #[cfg(target_os = "windows")]
    {
        let _ = app;
        task()
    }
}
#[tauri::command]
pub(crate) fn get_floating_hotkey(app: tauri::AppHandle) -> Status {
    app.state::<Service>().status()
}
#[tauri::command]
pub(crate) async fn set_floating_hotkey(
    app: tauri::AppHandle,
    shortcut: Option<Shortcut>,
) -> Result<Status, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let dispatch = app.clone();
        on_thread(&dispatch, move || {
            let result = app.state::<Service>().save(shortcut);
            publish(&app);
            result?;
            Ok(app.state::<Service>().status())
        })
    })
    .await
    .map_err(|_| "快捷键服务中断".to_string())?
}
#[tauri::command]
pub(crate) async fn record_floating_hotkey(
    app: tauri::AppHandle,
    recording: bool,
) -> Result<Status, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let dispatch = app.clone();
        on_thread(&dispatch, move || {
            let generation = app.state::<Service>().record(recording)?;
            publish(&app);
            if recording {
                let lease = app.clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_secs(30));
                    let dispatch = lease.clone();
                    let _ = on_thread(&dispatch, move || {
                        lease.state::<Service>().expire(generation);
                        publish(&lease);
                        Ok(())
                    });
                });
            }
            Ok(app.state::<Service>().status())
        })
    })
    .await
    .map_err(|_| "快捷键服务中断".to_string())?
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn persisted_slots_replace_restore_and_survive_interrupted_write() {
        let root = std::env::temp_dir().join(format!("tokendaily-hotkey-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let path = root.join("preference.json");
        let a = Some(Shortcut {
            key: 119,
            modifiers: 0,
        });
        let b = Some(Shortcut {
            key: 75,
            modifiers: 6,
        });
        persist(&path, a).unwrap();
        persist(&path, b).unwrap();
        assert_eq!(read_saved(&path).shortcut, b);
        let latest = read_saved(&path).generation as usize % 2;
        fs::write(&slots(&path)[latest], b"{\"generation\":").unwrap();
        assert_eq!(read_saved(&path).shortcut, a);
        persist(&path, None).unwrap();
        assert_eq!(read_saved(&path).shortcut, None);
        let service = Service {
            inner: Mutex::new(fixture()),
            path: path.clone(),
        };
        service.clear().unwrap();
        assert!(slots(&path).iter().all(|path| !path.exists()));
        assert_eq!(read_saved(&path).shortcut, None);
        fs::remove_dir(root).unwrap();
    }
    struct Fake {
        active: Option<Shortcut>,
    }
    impl Registry for Fake {
        fn replace(&mut self, value: Option<Shortcut>) -> Result<(), String> {
            if value.is_some_and(|key| key.key == 120) {
                return Err("conflict".into());
            }
            self.active = value;
            Ok(())
        }
    }
    fn fixture() -> Inner {
        Inner {
            registry: Box::new(Fake { active: None }),
            status: Status {
                shortcut: Some(Shortcut {
                    key: 119,
                    modifiers: 0,
                }),
                supported: true,
                active: true,
                recording: false,
                error: None,
            },
            enabled: true,
            generation: 0,
        }
    }
    #[test]
    fn validates_single_combination_and_rejects_modifiers_reserved_and_unknown() {
        for modifiers in [0, 2, 6, 15] {
            assert!(Shortcut { key: 75, modifiers }.validate().is_ok());
        }
        for key in [0, 16, 17, 18, 91, 123, 255] {
            assert!(Shortcut { key, modifiers: 0 }.validate().is_err());
        }
        assert!(Shortcut {
            key: 119,
            modifiers: 16
        }
        .validate()
        .is_err());
    }
    #[test]
    fn conflict_or_persistence_failure_restores_original_and_ends_recording() {
        let mut c = fixture();
        c.status.recording = true;
        assert!(c
            .save(
                Some(Shortcut {
                    key: 120,
                    modifiers: 0
                }),
                || panic!("conflict must not persist")
            )
            .is_err());
        assert_eq!(c.status.shortcut.unwrap().key, 119);
        assert!(c.status.active);
        assert!(!c.status.recording);
        assert!(c
            .save(
                Some(Shortcut {
                    key: 118,
                    modifiers: 2
                }),
                || Err("disk".into())
            )
            .is_err());
        assert_eq!(c.status.shortcut.unwrap().key, 119);
        assert!(c.status.active);
        c.save(None, || Ok(())).unwrap();
        assert!(c.status.shortcut.is_none());
        assert!(!c.status.active);
    }
    #[test]
    fn disabled_keeps_preference_but_releases_key_and_old_leases_do_not_resume() {
        let service = Service {
            inner: Mutex::new(fixture()),
            path: PathBuf::new(),
        };
        service.enabled(false);
        assert!(!service.status().active);
        assert!(service.status().shortcut.is_some());
        service.enabled(true);
        assert!(service.status().active);
        let old = service.record(true).unwrap();
        let new = service.record(true).unwrap();
        service.expire(old);
        assert!(service.status().recording);
        service.expire(new);
        assert!(!service.status().recording);
        assert!(service.status().active);
    }
}
