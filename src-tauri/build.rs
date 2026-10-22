fn main() {
    // Keep the Windows executable resource in sync with the brand asset.
    // Tauri's generated build output otherwise only watches tauri.conf.json,
    // which can leave a stale icon embedded in a freshly rebuilt installer.
    println!("cargo:rerun-if-changed=icons/icon.ico");
    println!("cargo:rerun-if-changed=icons/tray/64x64.png");
    tauri_build::build()
}
