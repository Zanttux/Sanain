//! Disk-full drill on a real, nearly full filesystem.
//!
//! Gated by `TEXT3_FULL_DIR` (or the old `SANAIN_FULL_DIR`), which must point to a directory on a small
//! throwaway filesystem (a 1 MiB tmpfs on Linux CI, a small VHD on Windows,
//! a small disk image on macOS; see docs/TESTING.md). Without it the test
//! prints "skipped" and passes. The test fills the filesystem itself, leaves
//! [`SLACK`] bytes free, and checks that every write Text³ makes fails
//! cleanly when it does not fit:
//!
//! - Save from the editor (atomic rename, and on Unix in place for
//!   hard-linked files) reports a disk-full I/O error, keeps the file's bytes, leaves no
//!   temporary or backup file behind and keeps the unsaved text in memory.
//! - Recovery snapshots, settings, session and macros writes fail without
//!   touching the previous files.
//! - A small save still succeeds on the nearly full disk, and a large one
//!   once the space is freed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use text3_app::editor::Command;
use text3_app::macros::MacroStore;
use text3_app::session::Session;
use text3_app::settings::Settings;
use text3_app::{Editor, SaveOutcome};
use text3_io::SaveError;
use text3_io::TextEncoding;
use text3_io::recovery::{RecoveryStore, Snapshot};

/// Free space left after filling: room for small writes and for the
/// in-place save's backup copy, far less than [`BIG`].
const SLACK: u64 = 64 * 1024;
/// Size of the writes that must not fit.
const BIG: usize = 1024 * 1024;
/// Refuse to fill more than this: the variable must name a small disk.
const MAX_FILL: u64 = 512 * 1024 * 1024;

/// The drill's working directory; removed (with the ballast) on drop so a
/// failed run frees the disk again.
struct Drill(PathBuf);

impl Drop for Drill {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// A file that takes up all free space but [`SLACK`].
struct Ballast {
    path: PathBuf,
}

impl Ballast {
    fn fill(path: PathBuf) -> Self {
        let mut f = fs::File::create(&path).unwrap();
        let chunk = vec![0xA5u8; 4096];
        let mut written = 0u64;
        loop {
            match f.write(&chunk) {
                Ok(0) => break,
                Ok(n) => written += n as u64,
                Err(e) if is_disk_full(&e) => break,
                Err(e) => panic!("filling the disk: {e}"),
            }
            if written > MAX_FILL {
                drop(f);
                let _ = fs::remove_file(&path);
                panic!("TEXT3_FULL_DIR has over {MAX_FILL} bytes free; use a small filesystem");
            }
        }
        // Make sure the space is really allocated; a delayed allocation
        // could otherwise fail here, which also means full.
        let _ = f.sync_all();
        assert!(
            written > SLACK,
            "filesystem too small for the drill: only {written} bytes were free"
        );
        f.set_len(written - SLACK).unwrap();
        f.sync_all().unwrap();
        Self { path }
    }
}

impl Drop for Ballast {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn is_disk_full(e: &io::Error) -> bool {
    matches!(
        e.kind(),
        io::ErrorKind::StorageFull | io::ErrorKind::QuotaExceeded
    )
}

/// Every file under `dir` but the ballast, with its bytes.
fn tree(dir: &Path, skip: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut out = BTreeMap::new();
    let mut stack = vec![dir.to_owned()];
    while let Some(d) = stack.pop() {
        for entry in fs::read_dir(&d).unwrap() {
            let p = entry.unwrap().path();
            if p == skip {
                continue;
            }
            if p.is_dir() {
                stack.push(p);
            } else {
                let bytes = fs::read(&p).unwrap();
                out.insert(p.strip_prefix(dir).unwrap().to_owned(), bytes);
            }
        }
    }
    out
}

fn set_text(ed: &mut Editor, text: &str) {
    ed.execute(Command::SelectAll);
    ed.execute(Command::Insert(text.to_owned()));
}

fn expect_disk_full(what: &str, outcome: SaveOutcome) {
    match outcome {
        SaveOutcome::Failed(SaveError::Io { source, .. }) if is_disk_full(&source) => {}
        other => panic!("{what}: expected a disk-full error, got {other:?}"),
    }
}

fn expect_io_disk_full<T: std::fmt::Debug>(what: &str, r: io::Result<T>) {
    match r {
        Err(e) if is_disk_full(&e) => {}
        other => panic!("{what}: expected a disk-full error, got {other:?}"),
    }
}

#[test]
fn writes_fail_cleanly_on_a_full_disk() {
    let Some(root) = text3_app::migrate::var_os("FULL_DIR") else {
        eprintln!("skipped: set TEXT3_FULL_DIR to a directory on a small filesystem");
        return;
    };
    let drill = Drill(PathBuf::from(root).join(format!("text3-drill-{}", std::process::id())));
    let dir = drill.0.clone();
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    // Files as they were before the disk filled up.
    let original = "original line é\r\n".repeat(128);
    let atomic = dir.join("atomic.txt");
    fs::write(&atomic, &original).unwrap();
    let linked = dir.join("linked.txt");
    let alias = dir.join("linked-alias.txt");
    fs::write(&linked, &original).unwrap();
    fs::hard_link(&linked, &alias).unwrap();

    let store = RecoveryStore::open(dir.join("recovery")).unwrap();
    let id = RecoveryStore::new_id();
    let old_snapshot = Snapshot {
        path: Some(atomic.clone()),
        encoding: TextEncoding::Utf8,
        text: "checkpoint before the disk filled".into(),
    };
    store.write(&id, &old_snapshot).unwrap();

    let config = dir.join("config");
    Settings::default().save(&config).unwrap();
    let session_file = config.join("session");
    Session {
        paths: vec![atomic.clone(), linked.clone()],
        ..Session::default()
    }
    .save(&session_file)
    .unwrap();
    let mut macros = MacroStore::default();
    macros
        .insert("small", &[Command::Insert("hi".into())])
        .unwrap();
    macros.save(&config).unwrap();
    // A macros file this version cannot read is backed up before it is
    // replaced; that backup must not be left half-written either. Larger
    // than SLACK so the copy cannot fit.
    let newer = dir.join("config-newer");
    fs::create_dir_all(&newer).unwrap();
    let mut future = String::from("sanain-macros 99\n");
    while future.len() < 2 * SLACK as usize {
        future.push_str("a macro from a newer version\n");
    }
    fs::write(newer.join("macros"), &future).unwrap();

    let big: String = "x".repeat(BIG - 1) + "\n";
    let mut ed_atomic = Editor::open(&atomic).unwrap();
    let mut ed_linked = Editor::open(&linked).unwrap();
    set_text(&mut ed_atomic, &big);
    set_text(&mut ed_linked, &big);

    let ballast_path = dir.join("ballast");
    let before = tree(&dir, &ballast_path);
    let ballast = Ballast::fill(ballast_path.clone());

    // Everything below must fail and change nothing.
    expect_disk_full("atomic save", ed_atomic.save());
    expect_disk_full("in-place save of a hard-linked file", ed_linked.save());
    for (name, ed) in [("atomic", &ed_atomic), ("linked", &ed_linked)] {
        assert!(ed.document().text() == big, "{name}: unsaved text kept");
        assert!(ed.document().is_dirty(), "{name}: still marked unsaved");
    }
    let big_snapshot = Snapshot {
        text: big.clone(),
        ..old_snapshot.clone()
    };
    expect_io_disk_full("recovery snapshot", store.write(&id, &big_snapshot));
    let big_settings = Settings {
        editor_font: Some("F".repeat(BIG)),
        ..Settings::default()
    };
    expect_io_disk_full("settings", big_settings.save(&config));
    let big_session = Session {
        paths: (0..BIG / 256)
            .map(|i| dir.join(format!("{i}-{}", "p".repeat(250))))
            .collect(),
        ..Session::default()
    };
    expect_io_disk_full("session", big_session.save(&session_file));
    let mut big_macros = MacroStore::default();
    big_macros
        .insert("big", &[Command::Insert("m".repeat(BIG / 2))])
        .unwrap();
    expect_io_disk_full("macros", big_macros.save(&config));
    expect_io_disk_full("macros with backup", big_macros.save(&newer));

    let after = tree(&dir, &ballast_path);
    let names = |t: &BTreeMap<PathBuf, Vec<u8>>| t.keys().cloned().collect::<Vec<_>>();
    assert_eq!(names(&before), names(&after), "no files added or removed");
    for (path, bytes) in &before {
        assert!(&after[path] == bytes, "{} changed", path.display());
    }
    let entries = store.list().unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].as_ref().unwrap().snapshot, old_snapshot);

    // A small save still fits on the nearly full disk, and the failed
    // attempt did not make the file look changed on disk.
    set_text(&mut ed_atomic, "small\r\n");
    assert!(matches!(ed_atomic.save(), SaveOutcome::Saved));
    assert_eq!(fs::read(&atomic).unwrap(), b"small\r\n");
    set_text(&mut ed_linked, "small linked\n");
    assert!(matches!(ed_linked.save(), SaveOutcome::Saved));
    // Saved in place on Unix, so the other name sees it; elsewhere the
    // atomic rename splits the link (a documented limit).
    #[cfg(unix)]
    assert_eq!(fs::read(&alias).unwrap(), b"small linked\n");
    let small_snapshot = Snapshot {
        text: "small".into(),
        ..old_snapshot.clone()
    };
    store.write(&id, &small_snapshot).unwrap();
    Settings {
        tab_width: 2,
        ..Settings::default()
    }
    .save(&config)
    .unwrap();
    assert_eq!(Settings::load(&config).tab_width, 2);

    // With the space freed, the large save goes through.
    drop(ballast);
    set_text(&mut ed_atomic, &big);
    assert!(matches!(ed_atomic.save(), SaveOutcome::Saved));
    assert!(fs::read(&atomic).unwrap() == big.as_bytes());
    big_macros.save(&newer).unwrap();
    assert!(
        newer.join("macros.1.bak").exists(),
        "unreadable macros kept"
    );
}
