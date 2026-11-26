//! Register Sanain with the desktop's file manager for the current user
//! (`sanain --register` / `--unregister`). No administrator rights; undone
//! by `--unregister`; never makes Sanain the default application.
//!
//! - Linux and other freedesktop systems: a `.desktop` entry in
//!   `$XDG_DATA_HOME/applications` listing the text MIME types, so file
//!   managers offer "Open With Sanain".
//! - Windows: per-user registry keys (HKCU) for "Edit with Sanain" in the
//!   Explorer context menu of every file and an "Open with" entry, written
//!   with `reg.exe`.
//! - macOS: not yet; Finder sends files as Apple events, which the window
//!   library does not deliver (needs an ADR for a small unsafe shim).

use std::path::{Path, PathBuf};

/// File name of the desktop entry (reverse-DNS, as freedesktop recommends).
pub const DESKTOP_FILE: &str = "io.github.zanttux.Sanain.desktop";

/// Icon name in the hicolor theme (installed by [`register_freedesktop`]).
pub const ICON_NAME: &str = "io.github.zanttux.Sanain";
const ICON_PNG: &[u8] = include_bytes!("../../../packaging/icons/sanain-256.png");

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
        "[Desktop Entry]\nType=Application\nName=Sanain\nGenericName=Text Editor\n\
         Comment=Native text editor\nExec={quoted} %F\nIcon={ICON_NAME}\n\
         Terminal=false\nCategories=Utility;TextEditor;Development;\n\
         MimeType={};\nStartupNotify=true\nStartupWMClass=sanain\n",
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
    sanain_io::write_atomic(&path, desktop_entry(exe).as_bytes(), None)?;
    if let Some(icon) = icon_path(&dir) {
        if let Some(parent) = icon.parent() {
            std::fs::create_dir_all(parent)?;
        }
        sanain_io::write_atomic(&icon, ICON_PNG, None)?;
    }
    refresh_desktop_database(&dir);
    Ok(path)
}

/// Remove the desktop entry; returns whether one existed.
pub fn unregister_freedesktop(apps: Option<PathBuf>) -> std::io::Result<bool> {
    let Some(dir) = apps.or_else(applications_dir) else {
        return Ok(false);
    };
    if let Some(icon) = icon_path(&dir) {
        let _ = std::fs::remove_file(icon);
    }
    match std::fs::remove_file(dir.join(DESKTOP_FILE)) {
        Ok(()) => {
            refresh_desktop_database(&dir);
            Ok(true)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e),
    }
}

/// `<data>/icons/hicolor/256x256/apps/<name>.png` next to `<data>/applications`.
fn icon_path(apps: &Path) -> Option<PathBuf> {
    apps.parent().map(|data| {
        data.join("icons/hicolor/256x256/apps")
            .join(format!("{ICON_NAME}.png"))
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

/// `reg.exe` argument lists that register (`true`) or remove the Windows
/// per-user keys for `exe`.
pub fn windows_reg_commands(exe: &Path, register: bool) -> Vec<Vec<String>> {
    let exe = exe.to_string_lossy().into_owned();
    let name = Path::new(&exe).file_name().map_or_else(
        || "sanain.exe".to_owned(),
        |n| n.to_string_lossy().into_owned(),
    );
    let menu = r"HKCU\Software\Classes\*\shell\Sanain".to_owned();
    let app = format!(r"HKCU\Software\Classes\Applications\{name}");
    let command = format!("\"{exe}\" \"%1\"");
    let s = |v: &[&str]| v.iter().map(|x| (*x).to_owned()).collect::<Vec<_>>();
    if register {
        vec![
            s(&["add", &menu, "/ve", "/d", "Edit with Sanain", "/f"]),
            s(&["add", &menu, "/v", "Icon", "/d", &exe, "/f"]),
            s(&[
                "add",
                &format!(r"{menu}\command"),
                "/ve",
                "/d",
                &command,
                "/f",
            ]),
            s(&["add", &app, "/v", "FriendlyAppName", "/d", "Sanain", "/f"]),
            s(&[
                "add",
                &format!(r"{app}\shell\open\command"),
                "/ve",
                "/d",
                &command,
                "/f",
            ]),
        ]
    } else {
        vec![s(&["delete", &menu, "/f"]), s(&["delete", &app, "/f"])]
    }
}

/// Run the Windows registration commands. Removing keys that do not exist
/// is not an error.
pub fn windows_apply(exe: &Path, register: bool) -> std::io::Result<()> {
    for args in windows_reg_commands(exe, register) {
        let status = std::process::Command::new("reg")
            .args(&args)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()?;
        if register && !status.success() {
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
        let e = desktop_entry(Path::new("/opt/my apps/$x/sanain"));
        assert!(e.contains("Exec=\"/opt/my apps/\\$x/sanain\" %F\n"));
        assert!(e.contains("MimeType=text/plain;text/markdown;"));
        assert!(e.starts_with("[Desktop Entry]\nType=Application\nName=Sanain\n"));
    }

    #[test]
    fn freedesktop_register_round_trip() {
        let d = tempfile::tempdir().unwrap();
        let apps = d.path().join("applications");
        let p = register_freedesktop(Path::new("/usr/bin/sanain"), Some(apps.clone())).unwrap();
        assert!(
            std::fs::read_to_string(&p)
                .unwrap()
                .contains("Exec=\"/usr/bin/sanain\" %F")
        );
        let icon = d
            .path()
            .join("icons/hicolor/256x256/apps/io.github.zanttux.Sanain.png");
        assert!(icon.is_file(), "icon installed");
        assert!(unregister_freedesktop(Some(apps.clone())).unwrap());
        assert!(!p.exists());
        assert!(!icon.exists(), "icon removed");
        assert!(
            !unregister_freedesktop(Some(apps)).unwrap(),
            "second time: nothing to remove"
        );
    }

    #[test]
    fn windows_commands() {
        let cmds = windows_reg_commands(Path::new(r"C:\Apps\Sanain\sanain.exe"), true);
        assert!(
            cmds.iter()
                .any(|c| c.contains(&r#""C:\Apps\Sanain\sanain.exe" "%1""#.to_owned()))
        );
        assert!(
            cmds.iter().all(|c| c[1].starts_with(r"HKCU\")),
            "per user only"
        );
        let undo = windows_reg_commands(Path::new(r"C:\Apps\Sanain\sanain.exe"), false);
        assert_eq!(undo.len(), 2);
    }

    /// Writes real per-user keys, so only on CI runners (throwaway VMs).
    #[cfg(windows)]
    #[test]
    fn windows_registry_round_trip_on_ci() {
        if std::env::var_os("CI").is_none() {
            return;
        }
        let exe = Path::new(r"C:\Temp\sanain-test\sanain.exe");
        windows_apply(exe, true).unwrap();
        let out = std::process::Command::new("reg")
            .args([
                "query",
                r"HKCU\Software\Classes\*\shell\Sanain\command",
                "/ve",
            ])
            .output()
            .unwrap();
        assert!(String::from_utf8_lossy(&out.stdout).contains(r"C:\Temp\sanain-test\sanain.exe"));
        windows_apply(exe, false).unwrap();
        let gone = std::process::Command::new("reg")
            .args(["query", r"HKCU\Software\Classes\*\shell\Sanain"])
            .status()
            .unwrap();
        assert!(!gone.success());
    }
}
