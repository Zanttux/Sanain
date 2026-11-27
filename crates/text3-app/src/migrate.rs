//! Compatibility with Sanain, the product's name before Text³ (D27).
//!
//! - Environment variables: `TEXT3_X` is read first, then the old
//!   `SANAIN_X`, so existing scripts and shortcuts keep working.
//! - Data folders: on the first start, the old per-user folders
//!   (settings, macros, themes, recent files, crash recovery, session,
//!   local history) are copied to the new `text3` folders. Copy, never
//!   move: the old folder stays as it was and doubles as the backup, so
//!   Sanain still works and going back loses nothing. A file that already
//!   exists in the new folder is never overwritten. A marker file in the
//!   new folder records the copy, so it happens (and is announced) once.
//!   Folders chosen with `TEXT3_*`/`SANAIN_*` variables or portable mode
//!   are not moved by the rename and are left alone.

use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Marker written into a new folder after its contents were copied.
pub const MARKER: &str = "migrated-from-sanain";

/// `TEXT3_<name>`, or the legacy `SANAIN_<name>` when that is unset.
/// Empty values count as unset.
pub fn var_os(name: &str) -> Option<OsString> {
    first_set([
        std::env::var_os(format!("TEXT3_{name}")),
        std::env::var_os(format!("SANAIN_{name}")),
    ])
}

/// The first value that is set and not empty.
fn first_set(values: [Option<OsString>; 2]) -> Option<OsString> {
    values.into_iter().flatten().find(|v| !v.is_empty())
}

/// [`var_os`] as a UTF-8 string (non-UTF-8 values count as unset).
pub fn var(name: &str) -> Option<String> {
    var_os(name).and_then(|v| v.into_string().ok())
}

/// What [`copy_legacy_dir`] did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// No old folder (fresh install), or old and new are the same folder.
    NothingToDo,
    /// The marker says this folder was already migrated.
    AlreadyDone,
    /// Files were copied; the marker is now in place.
    Copied(Report),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Report {
    /// Files copied from the old folder.
    pub copied: usize,
    /// Files left alone because the new folder already had them.
    pub kept: usize,
    /// Entries not copied: symbolic links and other non-regular files.
    pub skipped: usize,
}

/// Copy everything in `old` that `new` does not have yet, then write the
/// marker. Idempotent: once the marker exists nothing happens. `old` is
/// only read. On an error (e.g. disk full) the marker is not written, no
/// partial file is left behind, and the next start tries again; files
/// already copied are then kept, not copied twice.
pub fn copy_legacy_dir(old: &Path, new: &Path) -> io::Result<Outcome> {
    if !old.is_dir() || same_dir(old, new) {
        return Ok(Outcome::NothingToDo);
    }
    if new.join(MARKER).exists() {
        return Ok(Outcome::AlreadyDone);
    }
    let mut report = Report::default();
    copy_tree(old, new, &mut report)?;
    let note = format!(
        "Copied from {} by Text³ {} (first start after the rename from Sanain).\n\
         The old folder was left unchanged. Delete this file to copy again.\n\
         copied={}\nkept={}\nskipped={}\n",
        old.display(),
        env!("CARGO_PKG_VERSION"),
        report.copied,
        report.kept,
        report.skipped
    );
    text3_io::write_atomic(&new.join(MARKER), note.as_bytes(), None)?;
    Ok(Outcome::Copied(report))
}

fn same_dir(a: &Path, b: &Path) -> bool {
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(a), Ok(b)) => a == b,
        _ => a == b,
    }
}

fn copy_tree(from: &Path, to: &Path, report: &mut Report) -> io::Result<()> {
    if !to.is_dir() {
        fs::create_dir_all(to)?;
        // Keep owner-only folders (recovery) owner-only.
        if let Ok(meta) = fs::metadata(from) {
            let _ = fs::set_permissions(to, meta.permissions());
        }
    }
    let mut entries: Vec<_> = fs::read_dir(from)?.collect::<Result<_, _>>()?;
    entries.sort_by_key(fs::DirEntry::file_name);
    for entry in entries {
        let kind = entry.file_type()?;
        let src = entry.path();
        let dst = to.join(entry.file_name());
        if kind.is_dir() {
            copy_tree(&src, &dst, report)?;
        } else if !kind.is_file() {
            report.skipped += 1;
        } else if dst.symlink_metadata().is_ok() {
            report.kept += 1;
        } else {
            copy_file(&src, &dst)?;
            report.copied += 1;
        }
    }
    Ok(())
}

/// Copy through a temporary name, so an interrupted copy never leaves a
/// truncated file under the real name (which a retry would then keep).
fn copy_file(src: &Path, dst: &Path) -> io::Result<()> {
    let mut tmp_name = dst.file_name().unwrap_or_default().to_os_string();
    tmp_name.push(".text3-migrate.tmp");
    let tmp = dst.with_file_name(tmp_name);
    let result = fs::copy(src, &tmp).and_then(|_| {
        // Never replace a file that appeared in the meantime.
        if dst.symlink_metadata().is_ok() {
            return Err(io::Error::from(io::ErrorKind::AlreadyExists));
        }
        fs::rename(&tmp, dst)
    });
    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result
}

/// The old/new folder pairs to migrate for the current user: settings and
/// the folder holding crash recovery, the session and local history.
/// Folders set by environment variables or portable mode are skipped.
pub fn default_pairs() -> Vec<(PathBuf, PathBuf)> {
    let mut pairs = Vec::new();
    if crate::settings::portable_root().is_some() {
        return pairs;
    }
    if var_os("CONFIG_DIR").is_none()
        && let (Some(old), Some(new)) = (
            crate::settings::Settings::legacy_default_dir(),
            crate::settings::Settings::default_dir(),
        )
    {
        pairs.push((old, new));
    }
    if var_os("RECOVERY_DIR").is_none()
        && let (Some(old), Some(new)) = (
            text3_io::recovery::RecoveryStore::legacy_default_dir(),
            text3_io::recovery::RecoveryStore::default_dir(),
        )
        && let (Some(old), Some(new)) = (old.parent(), new.parent())
    {
        let pair = (old.to_path_buf(), new.to_path_buf());
        // macOS keeps settings and recovery under one folder.
        if !pairs.contains(&pair) {
            pairs.push(pair);
        }
    }
    pairs
}

/// Run the first-start copy for every pair. Returns the number of files
/// copied (0 when there was nothing to do); problems are reported on
/// stderr and never stop the start-up.
pub fn run(pairs: &[(PathBuf, PathBuf)]) -> usize {
    let mut copied = 0;
    for (old, new) in pairs {
        match copy_legacy_dir(old, new) {
            Ok(Outcome::Copied(r)) if r.copied + r.kept + r.skipped > 0 => {
                eprintln!(
                    "text3: copied {} files from {} to {} ({} already present, {} skipped); \
                     the old folder was left unchanged",
                    r.copied,
                    old.display(),
                    new.display(),
                    r.kept,
                    r.skipped
                );
                copied += r.copied;
            }
            Ok(_) => {}
            Err(e) => eprintln!(
                "text3: could not copy {} to {}: {e}",
                old.display(),
                new.display()
            ),
        }
    }
    copied
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    fn write(p: &Path, text: &str) {
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, text).unwrap();
    }

    fn read(p: &Path) -> String {
        fs::read_to_string(p).unwrap()
    }

    /// A Sanain folder like a real one: settings, macros, a theme, the
    /// session, a recovery snapshot and local history.
    fn sanain_folder(root: &Path) -> PathBuf {
        let old = root.join("Sanain");
        write(&old.join("settings"), "sanain-settings 1\nfont_size=18\n");
        write(&old.join("macros"), "sanain-macros 1\n");
        write(&old.join("themes/mine.theme"), "name=Mine\n");
        write(
            &old.join("session"),
            "sanain-session 1\nactive=0\npath=/a\n",
        );
        write(&old.join("Recovery/1.sanain-recovery"), "SANAINRC…");
        write(&old.join("History/ab/cd"), "old version");
        old
    }

    #[test]
    fn fresh_install_does_nothing() {
        let d = tempfile::tempdir().unwrap();
        let new = d.path().join("text3");
        let out = copy_legacy_dir(&d.path().join("Sanain"), &new).unwrap();
        assert_eq!(out, Outcome::NothingToDo);
        assert!(!new.exists(), "no folder created for a fresh install");
    }

    #[test]
    fn upgrade_copies_everything_and_keeps_the_old_folder() {
        let d = tempfile::tempdir().unwrap();
        let old = sanain_folder(d.path());
        let new = d.path().join("text3");
        let out = copy_legacy_dir(&old, &new).unwrap();
        assert_eq!(
            out,
            Outcome::Copied(Report {
                copied: 6,
                kept: 0,
                skipped: 0
            })
        );
        for f in [
            "settings",
            "macros",
            "themes/mine.theme",
            "session",
            "Recovery/1.sanain-recovery",
            "History/ab/cd",
        ] {
            assert_eq!(read(&new.join(f)), read(&old.join(f)), "{f}");
        }
        assert!(read(&new.join(MARKER)).contains("copied=6"));
        // Old data untouched: still there, no marker added to it.
        assert_eq!(
            read(&old.join("settings")),
            "sanain-settings 1\nfont_size=18\n"
        );
        assert!(!old.join(MARKER).exists());
        let leftovers: Vec<_> = walk(&new)
            .into_iter()
            .filter(|p| p.to_string_lossy().contains("text3-migrate"))
            .collect();
        assert!(leftovers.is_empty(), "{leftovers:?}");
    }

    #[test]
    fn repeated_start_does_not_copy_again() {
        let d = tempfile::tempdir().unwrap();
        let old = sanain_folder(d.path());
        let new = d.path().join("text3");
        assert!(matches!(
            copy_legacy_dir(&old, &new).unwrap(),
            Outcome::Copied(_)
        ));
        // The user changes settings in Text³, and Sanain writes its own.
        write(&new.join("settings"), "sanain-settings 1\nfont_size=12\n");
        write(&old.join("settings"), "sanain-settings 1\nfont_size=30\n");
        write(&old.join("macros-new"), "x");
        assert_eq!(copy_legacy_dir(&old, &new).unwrap(), Outcome::AlreadyDone);
        assert_eq!(
            read(&new.join("settings")),
            "sanain-settings 1\nfont_size=12\n"
        );
        assert!(!new.join("macros-new").exists());
        assert_eq!(run(&[(old, new)]), 0, "nothing announced the second time");
    }

    #[test]
    fn both_present_never_overwrites_new_files() {
        let d = tempfile::tempdir().unwrap();
        let old = sanain_folder(d.path());
        let new = d.path().join("text3");
        write(&new.join("settings"), "sanain-settings 1\nfont_size=11\n");
        write(&new.join("History/ab/cd"), "newer version");
        let out = copy_legacy_dir(&old, &new).unwrap();
        assert_eq!(
            out,
            Outcome::Copied(Report {
                copied: 4,
                kept: 2,
                skipped: 0
            })
        );
        assert_eq!(
            read(&new.join("settings")),
            "sanain-settings 1\nfont_size=11\n"
        );
        assert_eq!(read(&new.join("History/ab/cd")), "newer version");
        assert_eq!(read(&new.join("macros")), "sanain-macros 1\n");
        assert!(read(&new.join(MARKER)).contains("kept=2"));
    }

    #[test]
    fn same_folder_is_left_alone() {
        let d = tempfile::tempdir().unwrap();
        let old = sanain_folder(d.path());
        assert_eq!(copy_legacy_dir(&old, &old).unwrap(), Outcome::NothingToDo);
        assert!(!old.join(MARKER).exists());
    }

    #[cfg(unix)]
    #[test]
    fn keeps_owner_only_permissions_and_skips_symlinks() {
        use std::os::unix::fs::PermissionsExt;
        let d = tempfile::tempdir().unwrap();
        let old = sanain_folder(d.path());
        fs::set_permissions(old.join("Recovery"), fs::Permissions::from_mode(0o700)).unwrap();
        fs::set_permissions(
            old.join("Recovery/1.sanain-recovery"),
            fs::Permissions::from_mode(0o600),
        )
        .unwrap();
        std::os::unix::fs::symlink("/etc/passwd", old.join("link")).unwrap();
        let new = d.path().join("text3");
        let Outcome::Copied(r) = copy_legacy_dir(&old, &new).unwrap() else {
            panic!("expected a copy");
        };
        assert_eq!(r.skipped, 1);
        assert!(
            new.join("link").symlink_metadata().is_err(),
            "link not followed"
        );
        let mode = |p: &Path| fs::metadata(p).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode(&new.join("Recovery")), 0o700);
        assert_eq!(mode(&new.join("Recovery/1.sanain-recovery")), 0o600);
    }

    #[test]
    fn text3_variables_win_over_sanain_ones() {
        let v = |s: &str| Some(OsString::from(s));
        assert_eq!(first_set([v("/new"), v("/old")]), v("/new"));
        assert_eq!(first_set([None, v("/old")]), v("/old"), "SANAIN_ alias");
        assert_eq!(first_set([v(""), v("/old")]), v("/old"), "empty is unset");
        assert_eq!(first_set([None, None]), None);
        assert_eq!(var_os("SURELY_UNSET_7F3A_VARIABLE"), None);
    }

    fn walk(dir: &Path) -> Vec<PathBuf> {
        let mut out = Vec::new();
        for e in fs::read_dir(dir).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                out.extend(walk(&p));
            }
            out.push(p);
        }
        out
    }
}
