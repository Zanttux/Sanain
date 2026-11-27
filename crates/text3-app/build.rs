//! Windows: embed the application icon (packaging/windows/text3.rc).
//! macOS: embed an Info.plist in the executable (`__TEXT,__info_plist`), so
//! a binary run outside the app bundle (e.g. `cargo run`) still has the
//! bundle name "Text³" in the menu bar instead of the file name. Inside
//! Text³.app the bundle's own Info.plist applies.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    #[cfg(windows)]
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        // Icon shown by Explorer, the taskbar pin and the installer.
        println!("cargo:rerun-if-changed=../../packaging/windows/text3.rc");
        println!("cargo:rerun-if-changed=../../packaging/icons/text3.ico");
        embed_resource::compile("../../packaging/windows/text3.rc", embed_resource::NONE)
            .manifest_optional()
            .expect("embed the Windows icon");
    }
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("macos") {
        return;
    }
    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_default();
    let plist = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key><string>Text³</string>
  <key>CFBundleDisplayName</key><string>Text³</string>
  <key>CFBundleIdentifier</key><string>io.github.zanttux.sanain</string>
  <!-- Bundle id kept from Sanain on purpose (docs/REBRAND.md). -->
  <key>CFBundleShortVersionString</key><string>{version}</string>
  <key>CFBundleVersion</key><string>{version}</string>
  <key>NSHighResolutionCapable</key><true/>
</dict>
</plist>
"#
    );
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR"));
    let path = out.join("Info.plist");
    std::fs::write(&path, plist).expect("write Info.plist");
    println!(
        "cargo:rustc-link-arg-bins=-Wl,-sectcreate,__TEXT,__info_plist,{}",
        path.display()
    );
}
