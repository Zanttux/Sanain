//! Crash-recovery checkpoints for the open document.
//!
//! While the document has unsaved changes, a snapshot is written at most
//! [`INTERVAL`] after the first unsaved edit, then again after later edits.
//! Writes run on a background thread so the UI never waits on disk. When the
//! document becomes clean (saved, or changes discarded) the snapshot is
//! removed.
//!
//! Hot exit ([`Recovery::write_now`]) writes the current text through the
//! same thread and waits for the result, so it lands after any checkpoint
//! still queued and the caller knows whether it reached disk.

use std::path::Path;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::time::{Duration, Instant};

use text3_core::Revision;
use text3_io::recovery::{Entry, RecoveryStore, Snapshot};

use crate::Editor;

pub const INTERVAL: Duration = Duration::from_secs(2);

enum Job {
    /// The text travels as a rope snapshot (an O(1) clone) and becomes a
    /// string on the writer thread, so a checkpoint of a large document
    /// does not stall the UI.
    Write(String, PendingSnapshot),
    /// Write and report the result (hot exit).
    WriteNow(String, PendingSnapshot, Sender<std::io::Result<()>>),
    Remove(String),
    Flush(Sender<()>),
}

struct PendingSnapshot {
    path: Option<std::path::PathBuf>,
    encoding: text3_io::TextEncoding,
    base: Option<text3_io::Fingerprint>,
    rope: ropey::Rope,
}

impl PendingSnapshot {
    fn of(editor: &Editor) -> Self {
        Self {
            path: editor.file().map(|f| f.path.clone()),
            encoding: editor.encoding(),
            base: editor.file().map(|f| f.fingerprint().clone()),
            rope: editor.document().rope().clone(),
        }
    }

    fn snapshot(&self) -> Snapshot {
        Snapshot {
            path: self.path.clone(),
            encoding: self.encoding,
            text: self.rope.to_string(),
            base: self.base.clone(),
        }
    }
}

/// Why [`Recovery::write_now`] could not keep a buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeepError {
    /// Larger than a snapshot may be.
    TooLarge,
    /// The write failed (read-only or full disk, …) or did not finish in
    /// time; the reason is for display.
    Failed(String),
}

#[derive(Debug)]
pub struct Recovery {
    id: String,
    /// Revision of the last snapshot sent to the writer.
    written: Option<Revision>,
    /// Revision [`Self::write_now`] last confirmed on disk.
    confirmed: Option<Revision>,
    due: Option<Instant>,
    jobs: Sender<Job>,
    errors: Receiver<String>,
}

impl std::fmt::Debug for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Job::Write(id, _) => write!(f, "Write({id})"),
            Job::WriteNow(id, ..) => write!(f, "WriteNow({id})"),
            Job::Remove(id) => write!(f, "Remove({id})"),
            Job::Flush(_) => write!(f, "Flush"),
        }
    }
}

impl Recovery {
    /// Start the writer thread for `store`. `id` continues an existing
    /// snapshot (after restoring it) or starts a new one.
    pub fn start(store: RecoveryStore, id: Option<String>) -> Self {
        let (jobs, rx) = channel::<Job>();
        let (err_tx, errors) = channel();
        thread::Builder::new()
            .name("text3-recovery".into())
            .spawn(move || {
                for job in rx {
                    let result = match &job {
                        Job::Write(id, p) => store.write(id, &p.snapshot()),
                        Job::WriteNow(id, p, done) => {
                            let _ = done.send(store.write(id, &p.snapshot()));
                            Ok(())
                        }
                        Job::Remove(id) => store.remove(id),
                        Job::Flush(done) => {
                            let _ = done.send(());
                            Ok(())
                        }
                    };
                    if let Err(e) = result {
                        let _ = err_tx.send(e.to_string());
                    }
                }
            })
            .map_err(|e| eprintln!("text3: recovery disabled: {e}"))
            .ok();
        Self {
            id: id.unwrap_or_else(RecoveryStore::new_id),
            written: None,
            confirmed: None,
            due: None,
            jobs,
            errors,
        }
    }

    /// Call every frame. Returns how long until the next checkpoint is due,
    /// so the UI can schedule a repaint.
    pub fn tick(&mut self, editor: &Editor, now: Instant) -> Option<Duration> {
        let doc = editor.document();
        if !doc.is_dirty() {
            self.due = None;
            self.confirmed = None;
            if self.written.take().is_some() {
                let _ = self.jobs.send(Job::Remove(self.id.clone()));
            }
            return None;
        }
        if self.written == Some(doc.revision()) {
            return None;
        }
        let due = *self.due.get_or_insert(now + INTERVAL);
        if now < due {
            return Some(due - now);
        }
        let snapshot = PendingSnapshot::of(editor);
        let _ = self.jobs.send(Job::Write(self.id.clone(), snapshot));
        self.written = Some(doc.revision());
        self.due = None;
        None
    }

    /// Drop the snapshot (changes were discarded) and start a fresh one for
    /// the next document.
    pub fn discard(&mut self) {
        let _ = self.jobs.send(Job::Remove(self.id.clone()));
        self.id = RecoveryStore::new_id();
        self.written = None;
        self.confirmed = None;
        self.due = None;
    }

    /// Wait until queued writes and removals reach disk, at most `limit`.
    /// Call before the process exits.
    pub fn flush(&self, limit: Duration) {
        let (tx, rx) = channel();
        if self.jobs.send(Job::Flush(tx)).is_ok() {
            let _ = rx.recv_timeout(limit);
        }
    }

    /// Write the editor's current text now and wait (at most `limit`) until
    /// it is on disk, for hot exit. Texts longer than `max_len` bytes are
    /// not written. Earlier queued checkpoints land first, so they cannot
    /// replace this one.
    pub fn write_now(
        &mut self,
        editor: &Editor,
        max_len: usize,
        limit: Duration,
    ) -> Result<(), KeepError> {
        let doc = editor.document();
        if self.confirmed == Some(doc.revision()) {
            return Ok(()); // already on disk, nothing queued after it
        }
        // UTF-8 length without building the string.
        if doc.rope().len_bytes() > max_len {
            return Err(KeepError::TooLarge);
        }
        let (tx, rx) = channel();
        let job = Job::WriteNow(self.id.clone(), PendingSnapshot::of(editor), tx);
        if self.jobs.send(job).is_err() {
            return Err(KeepError::Failed("recovery is not running".into()));
        }
        match rx.recv_timeout(limit) {
            Ok(Ok(())) => {
                self.written = Some(doc.revision());
                self.confirmed = self.written;
                self.due = None;
                Ok(())
            }
            Ok(Err(e)) => Err(KeepError::Failed(e.to_string())),
            Err(_) => Err(KeepError::Failed("timed out".into())),
        }
    }

    /// The snapshot's identifier in the store.
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn take_error(&self) -> Option<String> {
        self.errors.try_iter().last()
    }
}

/// Choose the snapshot to restore at startup: the newest one for `path`
/// if a path was given, otherwise the newest overall. Damaged snapshots
/// are skipped (and left on disk).
pub fn pick(entries: Vec<Entry>, path: Option<&Path>) -> Option<Entry> {
    entries.into_iter().find(|e| match path {
        Some(p) => e.snapshot.path.as_deref() == Some(p),
        None => true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Command;

    fn wait_for(store: &RecoveryStore, n: usize) -> Vec<Entry> {
        for _ in 0..200 {
            let list: Vec<_> = store
                .list()
                .unwrap()
                .into_iter()
                .filter_map(Result::ok)
                .collect();
            if list.len() == n {
                return list;
            }
            thread::sleep(Duration::from_millis(10));
        }
        panic!("expected {n} snapshots");
    }

    #[test]
    fn checkpoint_after_interval_then_removed_when_clean() {
        let dir = tempfile::tempdir().unwrap();
        let store = RecoveryStore::open(dir.path()).unwrap();
        let mut rec = Recovery::start(store.clone(), None);
        let mut ed = Editor::new();
        let t0 = Instant::now();
        assert_eq!(rec.tick(&ed, t0), None, "clean: nothing to do");

        ed.execute(Command::Insert("draft".into()));
        assert_eq!(rec.tick(&ed, t0), Some(INTERVAL));
        // Further typing does not postpone the deadline.
        ed.execute(Command::Insert("!".into()));
        assert_eq!(
            rec.tick(&ed, t0 + Duration::from_secs(1)),
            Some(Duration::from_secs(1))
        );
        assert_eq!(rec.tick(&ed, t0 + INTERVAL), None);
        let list = wait_for(&store, 1);
        assert_eq!(list[0].snapshot.text, "draft!");

        let path = dir.path().join("saved.txt");
        assert!(matches!(ed.save_as(&path), crate::SaveOutcome::Saved));
        rec.tick(&ed, t0 + INTERVAL * 2);
        wait_for(&store, 0);
    }

    #[test]
    fn discard_removes_snapshot() {
        let dir = tempfile::tempdir().unwrap();
        let store = RecoveryStore::open(dir.path()).unwrap();
        let mut rec = Recovery::start(store.clone(), None);
        let mut ed = Editor::new();
        ed.execute(Command::Insert("x".into()));
        let t0 = Instant::now();
        rec.tick(&ed, t0);
        rec.tick(&ed, t0 + INTERVAL);
        wait_for(&store, 1);
        rec.discard();
        rec.flush(Duration::from_secs(5));
        assert_eq!(store.list().unwrap().len(), 0, "flush waits for removal");
    }

    #[test]
    fn restore_reopens_file_with_recovered_text_as_undoable_edit() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("doc.txt");
        std::fs::write(&path, "on disk\r\n").unwrap();
        let snap = Snapshot {
            path: Some(path.clone()),
            encoding: text3_io::TextEncoding::Utf8,
            text: "recovered\r\n".into(),
            base: None,
        };
        let mut ed = Editor::restore(&snap);
        assert_eq!(ed.document().text(), "recovered\r\n");
        assert!(ed.document().is_dirty());
        assert_eq!(ed.file_name().as_deref(), Some("doc.txt"));
        ed.execute(Command::Undo);
        assert_eq!(ed.document().text(), "on disk\r\n");
        assert!(!ed.document().is_dirty());
    }

    #[test]
    fn restore_of_a_deleted_file_keeps_its_path_and_save_recreates_it() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("gone.txt");
        let snap = Snapshot {
            path: Some(path.clone()),
            encoding: text3_io::TextEncoding::Utf8Bom,
            text: "keep me".into(),
            base: Some(text3_io::Fingerprint::from_parts(3, None, 1)),
        };
        let (mut ed, state) = Editor::restore_checked(&snap);
        assert_eq!(state, crate::editor::Restored::Missing);
        assert_eq!(ed.document().text(), "keep me");
        assert!(ed.document().is_dirty());
        assert!(ed.is_missing());
        assert_eq!(ed.file().map(|f| f.path.clone()), Some(path.clone()));
        assert_eq!(ed.encoding(), text3_io::TextEncoding::Utf8Bom);
        assert!(!path.exists(), "restoring writes nothing");

        // A file that appeared there meanwhile is not replaced silently.
        std::fs::write(&path, "someone else").unwrap();
        assert!(matches!(ed.save(), crate::SaveOutcome::Conflict));
        assert_eq!(std::fs::read(&path).unwrap(), b"someone else");
        std::fs::remove_file(&path).unwrap();
        assert!(matches!(ed.save(), crate::SaveOutcome::Saved));
        assert_eq!(std::fs::read(&path).unwrap(), b"\xEF\xBB\xBFkeep me");
        assert!(!ed.is_missing());
    }

    /// The disk state at load time, as a snapshot records it.
    fn base_of(path: &Path) -> text3_io::Fingerprint {
        Editor::open(path)
            .unwrap()
            .file()
            .unwrap()
            .fingerprint()
            .clone()
    }

    #[test]
    fn restore_after_an_external_change_keeps_both_and_save_asks() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("doc.txt");
        std::fs::write(&path, "original\n").unwrap();
        let snap = Snapshot {
            path: Some(path.clone()),
            encoding: text3_io::TextEncoding::Utf8,
            text: "original\nmy edit\n".into(),
            base: Some(base_of(&path)),
        };
        std::fs::write(&path, "their edit\n").unwrap();
        let (mut ed, state) = Editor::restore_checked(&snap);
        assert_eq!(state, crate::editor::Restored::ChangedOnDisk);
        assert_eq!(ed.document().text(), "original\nmy edit\n");
        assert!(ed.document().is_dirty());
        assert!(matches!(ed.save(), crate::SaveOutcome::Conflict));
        assert_eq!(std::fs::read(&path).unwrap(), b"their edit\n");
        // Undo shows what is on disk now.
        ed.execute(Command::Undo);
        assert_eq!(ed.document().text(), "their edit\n");
    }

    #[test]
    fn restore_on_an_unchanged_or_only_touched_file_applies_the_edits() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("doc.txt");
        std::fs::write(&path, "same\n").unwrap();
        let snap = Snapshot {
            path: Some(path.clone()),
            encoding: text3_io::TextEncoding::Utf8,
            text: "same\nmore\n".into(),
            base: Some(base_of(&path)),
        };
        // Same bytes, new modification time.
        thread::sleep(Duration::from_millis(20));
        std::fs::write(&path, "same\n").unwrap();
        let (mut ed, state) = Editor::restore_checked(&snap);
        assert_eq!(state, crate::editor::Restored::Applied);
        assert!(matches!(ed.save(), crate::SaveOutcome::Saved));
        assert_eq!(std::fs::read(&path).unwrap(), b"same\nmore\n");
    }

    #[test]
    fn restore_keeps_a_legacy_encoding_and_crlf_byte_exact() {
        let latin1 = text3_io::TextEncoding::Legacy(encoding_rs::WINDOWS_1252);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("vanha.txt");
        std::fs::write(&path, b"p\xe4iv\xe4\r\n").unwrap();
        let base = Editor::open_as(&path, latin1)
            .unwrap()
            .file()
            .unwrap()
            .fingerprint()
            .clone();
        let snap = Snapshot {
            path: Some(path.clone()),
            encoding: latin1,
            text: "p\u{e4}iv\u{e4}\r\nyö\r\n".into(),
            base: Some(base),
        };
        let (mut ed, state) = Editor::restore_checked(&snap);
        assert_eq!(state, crate::editor::Restored::Applied);
        assert_eq!(ed.encoding(), latin1);
        assert_eq!(ed.line_ending(), text3_core::LineEnding::CrLf);
        assert!(matches!(ed.save(), crate::SaveOutcome::Saved));
        assert_eq!(std::fs::read(&path).unwrap(), b"p\xe4iv\xe4\r\ny\xf6\r\n");
    }

    #[test]
    fn write_now_is_on_disk_when_it_returns() {
        let dir = tempfile::tempdir().unwrap();
        let store = RecoveryStore::open(dir.path()).unwrap();
        let mut rec = Recovery::start(store.clone(), None);
        let mut ed = Editor::new();
        ed.execute(Command::Insert("first".into()));
        let t0 = Instant::now();
        rec.tick(&ed, t0);
        rec.tick(&ed, t0 + INTERVAL); // a checkpoint is queued
        ed.execute(Command::Insert(" second".into()));
        rec.write_now(&ed, usize::MAX, Duration::from_secs(10))
            .unwrap();
        let list: Vec<_> = store
            .list()
            .unwrap()
            .into_iter()
            .map(|r| r.unwrap())
            .collect();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, rec.id());
        assert_eq!(list[0].snapshot.text, "first second", "not the queued one");
        // Nothing more to write for this revision.
        assert_eq!(rec.tick(&ed, t0 + INTERVAL * 3), None);
    }

    #[test]
    fn write_now_refuses_a_text_over_the_limit() {
        let dir = tempfile::tempdir().unwrap();
        let store = RecoveryStore::open(dir.path()).unwrap();
        let mut rec = Recovery::start(store.clone(), None);
        let mut ed = Editor::new();
        ed.execute(Command::Insert("ääää".into())); // 8 bytes
        assert_eq!(
            rec.write_now(&ed, 7, Duration::from_secs(10)),
            Err(KeepError::TooLarge)
        );
        rec.write_now(&ed, 8, Duration::from_secs(10)).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn write_now_reports_a_read_only_store() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let rdir = dir.path().join("r");
        let store = RecoveryStore::open(&rdir).unwrap();
        std::fs::set_permissions(&rdir, std::fs::Permissions::from_mode(0o500)).unwrap();
        if std::fs::write(rdir.join("probe"), b"x").is_ok() {
            return; // running as root: permissions are not enforced
        }
        let mut rec = Recovery::start(store, None);
        let mut ed = Editor::new();
        ed.execute(Command::Insert("x".into()));
        let r = rec.write_now(&ed, usize::MAX, Duration::from_secs(10));
        std::fs::set_permissions(&rdir, std::fs::Permissions::from_mode(0o700)).unwrap();
        assert!(matches!(r, Err(KeepError::Failed(_))), "{r:?}");
    }
}
