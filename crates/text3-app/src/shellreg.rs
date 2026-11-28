//! Register Text³ with the desktop's file manager for the current user
//! (`text3 --register` / `--unregister`). No administrator rights; undone
//! by `--unregister`; never makes Text³ the default application.
//!
//! - Linux and other freedesktop systems: a `.desktop` entry in
//!   `$XDG_DATA_HOME/applications` listing the text MIME types, so file
//!   managers offer "Open With Text³".
//! - Windows: per-user registry keys (HKCU) for "Edit with Text³" in the
//!   Explorer context menu of every file and an "Open with" entry, written
//!   with `reg.exe`, plus the ProgID and capabilities that list Text³ in
//!   Settings › Default apps ([`crate::defaults::windows_reg_commands`]).
//! - Both `--register` and `--unregister` also remove what Sanain (the
//!   name before Text³) registered, so no stale "Edit with Sanain" entry
//!   pointing at a removed program is left behind.
//! - macOS: nothing to register; Info.plist declares the document types and
//!   text3-platform handles Finder's open events (ADR 0003).

use std::path::{Path, PathBuf};

/// File name of the desktop entry (reverse-DNS, as freedesktop recommends).
pub const DESKTOP_FILE: &str = "io.github.zanttux.text3.desktop";

/// Icon name in the hicolor theme (installed by [`register_freedesktop`]).
pub const ICON_NAME: &str = "io.github.zanttux.text3";
const ICON_PNG: &[u8] = include_bytes!("../../../packaging/icons/text3-256.png");

/// Desktop entry and icon name written by Sanain before the rename.
pub const LEGACY_DESKTOP_FILE: &str = "io.github.zanttux.Sanain.desktop";
pub const LEGACY_ICON_NAME: &str = "io.github.zanttux.Sanain";

/// MIME types offered in "Open With": text and common code/data formats.
pub const MIME_TYPES: &[&str] = &[
    "text/plain",
    "text/markdown",
    "text/x-markdown",
    "text/csv",
    "text/tab-separated-values",
    "text/x-log",
    "text/xml",
    "application/xml",
    "application/json",
    "application/x-yaml",
    "application/toml",
    "text/x-csrc",
    "text/x-chdr",
    "text/x-c++src",
    "text/x-csharp",
    "text/x-go",
    "text/x-java",
    "text/x-python",
    "text/x-ruby",
    "text/rust",
    "text/x-shellscript",
    "application/x-shellscript",
    "text/css",
    "text/html",
    "text/javascript",
    "application/javascript",
    "text/x-typescript",
    "text/x-c++hdr",
    "text/x-python3",
    "text/x-lua",
    "application/sql",
    "text/x-patch",
    "text/x-diff",
    "application/yaml",
];

/// The `.desktop` entry for the executable at `exe`.
pub fn desktop_entry(exe: &Path) -> String {
    // Exec quoting per the Desktop Entry spec: double quotes, with `"`,
    // `` ` ``, `$` and `\` escaped by a backslash.
    let mut quoted = String::from("\"");
    for c in exe.to_string_lossy().chars() {
        if matches!(c, '"' | '`' | '$' | '\\') {
            quoted.push('\\');
        }
        quoted.push(c);
    }
    quoted.push('"');
    format!(
        "[Desktop Entry]\nType=Application\nName=Text³\nGenericName=Text Editor\n\
         Comment=Native text editor\nExec={quoted} %F\nIcon={ICON_NAME}\n\
         Terminal=false\nCategories=Utility;TextEditor;Development;\n\
         MimeType={};\nStartupNotify=true\nStartupWMClass=text3\n",
        MIME_TYPES.join(";")
    )
}

fn applications_dir() -> Option<PathBuf> {
    std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))
        .map(|d| d.join("applications"))
}

/// Write the desktop entry; returns its path.
pub fn register_freedesktop(exe: &Path, apps: Option<PathBuf>) -> std::io::Result<PathBuf> {
    let dir = apps
        .or_else(applications_dir)
        .ok_or_else(|| std::io::Error::other("no home folder"))?;
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(DESKTOP_FILE);
    text3_io::write_atomic(&path, desktop_entry(exe).as_bytes(), None)?;
    remove_legacy_freedesktop(&dir);
    if let Some(icon) = icon_path(&dir, ICON_NAME) {
        if let Some(parent) = icon.parent() {
            std::fs::create_dir_all(parent)?;
        }
        text3_io::write_atomic(&icon, ICON_PNG, None)?;
    }
    refresh_desktop_database(&dir);
    Ok(path)
}

/// Remove the desktop entry; returns whether one existed.
pub fn unregister_freedesktop(apps: Option<PathBuf>) -> std::io::Result<bool> {
    let Some(dir) = apps.or_else(applications_dir) else {
        return Ok(false);
    };
    if let Some(icon) = icon_path(&dir, ICON_NAME) {
        let _ = std::fs::remove_file(icon);
    }
    let legacy = remove_legacy_freedesktop(&dir);
    match std::fs::remove_file(dir.join(DESKTOP_FILE)) {
        Ok(()) => {
            refresh_desktop_database(&dir);
            Ok(true)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            if legacy {
                refresh_desktop_database(&dir);
            }
            Ok(legacy)
        }
        Err(e) => Err(e),
    }
}

/// Remove Sanain's desktop entry and icon; returns whether the entry existed.
fn remove_legacy_freedesktop(apps: &Path) -> bool {
    if let Some(icon) = icon_path(apps, LEGACY_ICON_NAME) {
        let _ = std::fs::remove_file(icon);
    }
    std::fs::remove_file(apps.join(LEGACY_DESKTOP_FILE)).is_ok()
}

/// `<data>/icons/hicolor/256x256/apps/<name>.png` next to `<data>/applications`.
fn icon_path(apps: &Path, name: &str) -> Option<PathBuf> {
    apps.parent().map(|data| {
        data.join("icons/hicolor/256x256/apps")
            .join(format!("{name}.png"))
    })
}

/// Best effort: file managers also notice the change without it.
fn refresh_desktop_database(dir: &Path) {
    let _ = std::process::Command::new("update-desktop-database")
        .arg(dir)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
}

/// A path without the Windows verbatim prefix (`\\?\C:\…` → `C:\…`,
/// `\\?\UNC\server\share` → `\\server\share`), which `canonicalize`
/// adds and Explorer cannot start. Other paths are returned unchanged.
pub fn plain_path(p: &Path) -> PathBuf {
    let s = p.to_string_lossy();
    if let Some(rest) = s.strip_prefix(r"\\?\UNC\") {
        return PathBuf::from(format!(r"\\{rest}"));
    }
    match s.strip_prefix(r"\\?\") {
        Some(rest) if rest.as_bytes().get(1) == Some(&b':') => PathBuf::from(rest),
        _ => p.to_path_buf(),
    }
}

/// Keys Sanain wrote before the rename; removed by register and unregister.
pub const LEGACY_WINDOWS_KEYS: [&str; 2] = [
    r"HKCU\Software\Classes\*\shell\Sanain",
    r"HKCU\Software\Classes\Applications\sanain.exe",
];

/// `reg.exe` argument lists that register (`true`) or remove the Windows
/// per-user keys for `exe`. Both also delete Sanain's old keys.
pub fn windows_reg_commands(exe: &Path, register: bool) -> Vec<Vec<String>> {
    let exe = exe.to_string_lossy().into_owned();
    let name = Path::new(&exe).file_name().map_or_else(
        || "text3.exe".to_owned(),
        |n| n.to_string_lossy().into_owned(),
    );
    let menu = r"HKCU\Software\Classes\*\shell\Text3".to_owned();
    let app = format!(r"HKCU\Software\Classes\Applications\{name}");
    let command = format!("\"{exe}\" \"%1\"");
    let s = |v: &[&str]| v.iter().map(|x| (*x).to_owned()).collect::<Vec<_>>();
    let legacy = LEGACY_WINDOWS_KEYS.iter().map(|k| s(&["delete", k, "/f"]));
    if register {
        let mut cmds = vec![
            s(&["add", &menu, "/ve", "/d", "Edit with Text³", "/f"]),
            s(&["add", &menu, "/v", "Icon", "/d", &exe, "/f"]),
            s(&[
                "add",
                &format!(r"{menu}\command"),
                "/ve",
                "/d",
                &command,
                "/f",
            ]),
            s(&["add", &app, "/v", "FriendlyAppName", "/d", "Text³", "/f"]),
            s(&[
                "add",
                &format!(r"{app}\shell\open\command"),
                "/ve",
                "/d",
                &command,
                "/f",
            ]),
        ];
        cmds.extend(legacy);
        cmds.extend(crate::defaults::windows_reg_commands(&exe, true));
        cmds
    } else {
        let mut cmds = vec![s(&["delete", &menu, "/f"]), s(&["delete", &app, "/f"])];
        cmds.extend(legacy);
        cmds.extend(crate::defaults::windows_reg_commands(&exe, false));
        cmds
    }
}

/// `reg.exe` in System32 by its full path. A bare `reg` would be looked up
/// in the folder of the running `text3.exe` first, where a downloaded
/// `reg.exe` could be waiting (Notepad++ CVE-2025-49144 class). There is
/// deliberately no fallback to the bare name.
fn reg_exe() -> std::io::Result<PathBuf> {
    text3_platform::system_program("reg.exe", text3_platform::SystemDir::System)
        .ok_or_else(|| std::io::Error::other("the Windows system folder was not found"))
}

/// Run the Windows registration commands. Removing keys that do not exist
/// is not an error (deletions are best effort, additions must succeed).
pub fn windows_apply(exe: &Path, register: bool) -> std::io::Result<()> {
    let reg = reg_exe()?;
    for args in windows_reg_commands(exe, register) {
        let status = std::process::Command::new(&reg)
            .args(&args)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()?;
        if args[0] == "add" && !status.success() {
            return Err(std::io::Error::other(format!(
                "reg {} failed",
                args.join(" ")
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desktop_entry_quotes_and_lists_types() {
        let e = desktop_entry(Path::new("/opt/my apps/$x/text3"));
        assert!(e.contains("Exec=\"/opt/my apps/\\$x/text3\" %F\n"));
        assert!(e.contains("MimeType=text/plain;text/markdown;"));
        assert!(e.starts_with("[Desktop Entry]\nType=Application\nName=Text³\n"));
    }

    #[test]
    fn freedesktop_register_round_trip() {
        let d = tempfile::tempdir().unwrap();
        let apps = d.path().join("applications");
        let p = register_freedesktop(Path::new("/usr/bin/text3"), Some(apps.clone())).unwrap();
        assert!(
            std::fs::read_to_string(&p)
                .unwrap()
                .contains("Exec=\"/usr/bin/text3\" %F")
        );
        let icon = d
            .path()
            .join("icons/hicolor/256x256/apps/io.github.zanttux.text3.png");
        assert!(icon.is_file(), "icon installed");
        assert!(unregister_freedesktop(Some(apps.clone())).unwrap());
        assert!(!p.exists());
        assert!(!icon.exists(), "icon removed");
        assert!(
            !unregister_freedesktop(Some(apps)).unwrap(),
            "second time: nothing to remove"
        );
    }

    /// Sanain's old entry and icon go away on register and on unregister.
    #[test]
    fn freedesktop_removes_the_sanain_entry() {
        let d = tempfile::tempdir().unwrap();
        let apps = d.path().join("applications");
        let old_icon = d
            .path()
            .join("icons/hicolor/256x256/apps/io.github.zanttux.Sanain.png");
        let plant = || {
            std::fs::create_dir_all(&apps).unwrap();
            std::fs::create_dir_all(old_icon.parent().unwrap()).unwrap();
            std::fs::write(apps.join(LEGACY_DESKTOP_FILE), "[Desktop Entry]\n").unwrap();
            std::fs::write(&old_icon, b"png").unwrap();
        };
        plant();
        register_freedesktop(Path::new("/usr/bin/text3"), Some(apps.clone())).unwrap();
        assert!(!apps.join(LEGACY_DESKTOP_FILE).exists());
        assert!(!old_icon.exists());
        assert!(apps.join(DESKTOP_FILE).is_file());
        unregister_freedesktop(Some(apps.clone())).unwrap();
        plant();
        assert!(
            unregister_freedesktop(Some(apps.clone())).unwrap(),
            "only the Sanain entry: still reported as removed"
        );
        assert!(!apps.join(LEGACY_DESKTOP_FILE).exists());
        assert!(!old_icon.exists());
    }

    #[test]
    fn verbatim_prefix_removed() {
        assert_eq!(
            plain_path(Path::new(
                r"\\?\C:\Users\Me\AppData\Local\Programs\Text3\text3.exe"
            )),
            PathBuf::from(r"C:\Users\Me\AppData\Local\Programs\Text3\text3.exe")
        );
        assert_eq!(
            plain_path(Path::new(r"\\?\UNC\server\share\text3.exe")),
            PathBuf::from(r"\\server\share\text3.exe")
        );
        assert_eq!(
            plain_path(Path::new("/usr/bin/text3")),
            PathBuf::from("/usr/bin/text3")
        );
    }

    #[test]
    fn windows_commands() {
        let cmds = windows_reg_commands(Path::new(r"C:\Apps\Text3\text3.exe"), true);
        assert!(
            cmds.iter()
                .any(|c| c.contains(&r#""C:\Apps\Text3\text3.exe" "%1""#.to_owned()))
        );
        assert!(
            cmds.iter().all(|c| c[1].starts_with(r"HKCU\")),
            "per user only"
        );
        let undo = windows_reg_commands(Path::new(r"C:\Apps\Text3\text3.exe"), false);
        assert_eq!(
            undo.len(),
            4 + crate::defaults::windows_reg_commands("x", false).len()
        );
        for key in LEGACY_WINDOWS_KEYS {
            let delete = vec!["delete".to_owned(), key.to_owned(), "/f".to_owned()];
            assert!(undo.contains(&delete), "unregister removes {key}");
            assert!(cmds.contains(&delete), "register removes {key}");
        }
        assert!(undo.iter().all(|c| c[0] == "delete"));
    }

    /// What `--register` writes: the canonical executable path without the
    /// verbatim prefix Explorer cannot run.
    #[cfg(windows)]
    #[test]
    fn registered_exe_path_is_plain() {
        let exe = std::env::current_exe().unwrap().canonicalize().unwrap();
        assert!(
            exe.to_string_lossy().starts_with(r"\\?\"),
            "canonicalize adds it"
        );
        let plain = plain_path(&exe);
        assert!(!plain.to_string_lossy().starts_with(r"\\?\"));
        assert!(
            plain.is_file(),
            "still names the executable: {}",
            plain.display()
        );
    }

    /// Writes real per-user keys, so only on CI runners (throwaway VMs).
    #[cfg(windows)]
    #[test]
    fn windows_registry_round_trip_on_ci() {
        if std::env::var_os("CI").is_none() {
            return;
        }
        let exe = Path::new(r"C:\Temp\text3-test\text3.exe");
        windows_apply(exe, true).unwrap();
        let out = std::process::Command::new(reg_exe().unwrap())
            .args([
                "query",
                r"HKCU\Software\Classes\*\shell\Text3\command",
                "/ve",
            ])
            .output()
            .unwrap();
        assert!(String::from_utf8_lossy(&out.stdout).contains(r"C:\Temp\text3-test\text3.exe"));
        windows_apply(exe, false).unwrap();
        let gone = std::process::Command::new(reg_exe().unwrap())
            .args(["query", r"HKCU\Software\Classes\*\shell\Text3"])
            .status()
            .unwrap();
        assert!(!gone.success());
    }
}
