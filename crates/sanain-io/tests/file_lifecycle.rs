//! First vertical slice: open, edit, undo, save atomically, reopen, verify
//! bytes. Plus conflict and round-trip guarantees.

#![allow(clippy::unwrap_used)]

use std::fs;

use sanain_core::{Change, Document, Transaction};
use sanain_io::{LoadError, SaveError, SaveMode, TextEncoding, encode, load, save};

#[test]
fn open_edit_undo_save_reopen_is_byte_exact() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("note.txt");
    let original = "first\r\nsecond 🙂\r\n";
    fs::write(&path, original).unwrap();

    let loaded = load(&path).unwrap();
    assert_eq!(loaded.encoding, TextEncoding::Utf8);
    let mut doc = Document::from_saved_text(&loaded.text);

    doc.apply(&Transaction::single(Change::insert(5, " line")))
        .unwrap();
    doc.apply(&Transaction::single(Change::insert(0, "TYPO")))
        .unwrap();
    assert!(doc.undo());
    assert!(doc.is_dirty());

    save(
        &path,
        &doc.text(),
        loaded.encoding,
        SaveMode::Expect(&loaded.fingerprint),
    )
    .unwrap();
    doc.mark_saved();

    assert_eq!(
        fs::read(&path).unwrap(),
        b"first line\r\nsecond \xF0\x9F\x99\x82\r\n"
    );
    let reopened = load(&path).unwrap();
    assert_eq!(reopened.text, doc.text());
    assert!(!doc.is_dirty());
}

#[test]
fn unmodified_save_round_trips_every_encoding() {
    let dir = tempfile::tempdir().unwrap();
    let samples: [(&str, Vec<u8>); 4] = [
        ("utf8.txt", "mixed\r\nends\nand\r é".as_bytes().to_vec()),
        ("bom.txt", b"\xEF\xBB\xBFbom\r\n".to_vec()),
        ("le.txt", encode("le\r\n漢🙂", TextEncoding::Utf16LeBom)),
        ("be.txt", encode("be\n", TextEncoding::Utf16BeBom)),
    ];
    for (name, bytes) in samples {
        let path = dir.path().join(name);
        fs::write(&path, &bytes).unwrap();
        let l = load(&path).unwrap();
        save(&path, &l.text, l.encoding, SaveMode::Expect(&l.fingerprint)).unwrap();
        assert_eq!(fs::read(&path).unwrap(), bytes, "{name}");
    }
}

#[test]
fn external_change_blocks_save_and_keeps_disk_content() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("shared.txt");
    fs::write(&path, "v1").unwrap();
    let l = load(&path).unwrap();

    fs::write(&path, "v2 from another program").unwrap();
    let err = save(&path, "mine", l.encoding, SaveMode::Expect(&l.fingerprint)).unwrap_err();
    assert!(matches!(err, SaveError::ExternalChange { .. }));
    assert_eq!(
        fs::read_to_string(&path).unwrap(),
        "v2 from another program"
    );

    // Deleted externally also counts as a change.
    fs::remove_file(&path).unwrap();
    let err = save(&path, "mine", l.encoding, SaveMode::Expect(&l.fingerprint)).unwrap_err();
    assert!(matches!(err, SaveError::ExternalChange { .. }));

    // Explicit overwrite after user confirmation succeeds.
    save(&path, "mine", l.encoding, SaveMode::Overwrite).unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), "mine");
}

#[test]
fn create_new_refuses_existing_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("exists.txt");
    fs::write(&path, "keep").unwrap();
    let err = save(&path, "x", TextEncoding::Utf8, SaveMode::CreateNew).unwrap_err();
    assert!(matches!(err, SaveError::ExternalChange { .. }));
    assert_eq!(fs::read_to_string(&path).unwrap(), "keep");

    let fresh = dir.path().join("new.txt");
    save(&fresh, "x", TextEncoding::Utf8, SaveMode::CreateNew).unwrap();
    assert_eq!(fs::read_to_string(&fresh).unwrap(), "x");
}

#[test]
fn invalid_utf8_load_fails_without_touching_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("latin1.txt");
    fs::write(&path, b"caf\xE9").unwrap();
    assert!(matches!(load(&path), Err(LoadError::Decode { .. })));
    assert_eq!(fs::read(&path).unwrap(), b"caf\xE9");
}

#[test]
fn save_leaves_no_temporary_files() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("a.txt");
    save(&path, "x", TextEncoding::Utf8, SaveMode::CreateNew).unwrap();
    let names: Vec<_> = fs::read_dir(dir.path())
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect();
    assert_eq!(names, ["a.txt"]);
}

#[cfg(unix)]
#[test]
fn save_preserves_permissions_and_symlinks() {
    use std::os::unix::fs::{PermissionsExt, symlink};
    let dir = tempfile::tempdir().unwrap();
    let real = dir.path().join("real.sh");
    let link = dir.path().join("link.sh");
    fs::write(&real, "#!/bin/sh\n").unwrap();
    fs::set_permissions(&real, fs::Permissions::from_mode(0o750)).unwrap();
    symlink(&real, &link).unwrap();

    let l = load(&link).unwrap();
    save(
        &link,
        "#!/bin/sh\necho hi\n",
        l.encoding,
        SaveMode::Expect(&l.fingerprint),
    )
    .unwrap();

    assert!(
        fs::symlink_metadata(&link)
            .unwrap()
            .file_type()
            .is_symlink()
    );
    assert_eq!(fs::read_to_string(&real).unwrap(), "#!/bin/sh\necho hi\n");
    assert_eq!(
        fs::metadata(&real).unwrap().permissions().mode() & 0o777,
        0o750
    );
}
