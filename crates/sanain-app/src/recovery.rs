//! Crash-recovery checkpoints for the open document.
//!
//! While the document has unsaved changes, a snapshot is written at most
//! [`INTERVAL`] after the first unsaved edit, then again after later edits.
//! Writes run on a background thread so the UI never waits on disk. When the
//! document becomes clean (saved, or changes discarded) the snapshot is
//! removed.

use std::path::Path;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::time::{Duration, Instant};

use sanain_core::Revision;
use sanain_io::recovery::{Entry, RecoveryStore, Snapshot};

use crate::Editor;

pub const INTERVAL: Duration = Duration::from_secs(2);

enum Job {
    /// The text travels as a rope snapshot (an O(1) clone) and becomes a
    /// string on the writer thread, so a checkpoint of a large document
    /// does not stall the UI.
    Write(String, PendingSnapshot),
    Remove(String),
    Flush(Sender<()>),
}

struct PendingSnapshot {
    path: Option<std::path::PathBuf>,
    encoding: sanain_io::TextEncoding,
    rope: ropey::Rope,
}

#[derive(Debug)]
pub struct Recovery {
    id: String,
    /// Revision of the last snapshot sent to the writer.
    written: Option<Revision>,
    due: Option<Instant>,
    jobs: Sender<Job>,
    errors: Receiver<String>,
}

impl std::fmt::Debug for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Job::Write(id, _) => write!(f, "Write({id})"),
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
            .name("sanain-recovery".into())
            .spawn(move || {
                for job in rx {
                    let result = match &job {
                        Job::Write(id, p) => store.write(
                            id,
                            &Snapshot {
                                path: p.path.clone(),
                                encoding: p.encoding,
                                text: p.rope.to_string(),
                            },
                        ),
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
            .map_err(|e| eprintln!("sanain: recovery disabled: {e}"))
            .ok();
        Self {
            id: id.unwrap_or_else(RecoveryStore::new_id),
            written: None,
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
        let snapshot = PendingSnapshot {
            path: editor.file().map(|f| f.path.clone()),
            encoding: editor.encoding(),
            rope: doc.rope().clone(),
        };
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
            encoding: sanain_io::TextEncoding::Utf8,
            text: "recovered\r\n".into(),
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
    fn restore_without_file_gives_untitled_dirty_buffer() {
        let snap = Snapshot {
            path: Some("/nonexistent/sanain/x.txt".into()),
            encoding: sanain_io::TextEncoding::Utf8Bom,
            text: "keep me".into(),
        };
        let ed = Editor::restore(&snap);
        assert_eq!(ed.document().text(), "keep me");
        assert!(ed.document().is_dirty());
        assert!(ed.file().is_none());
        assert_eq!(ed.encoding(), sanain_io::TextEncoding::Utf8Bom);
    }
}
