//! Open documents in one window: tabs, the active tab, recently closed
//! files and each tab's crash-recovery checkpoints. No GUI types.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use sanain_io::LoadError;
use sanain_io::recovery::{Entry, RecoveryStore};

use crate::Editor;
use crate::recovery::Recovery;

/// Stable identity of a tab for the lifetime of the process.
pub type TabId = u64;

#[derive(Debug)]
pub struct Tab {
    pub id: TabId,
    pub editor: Editor,
    recovery: Option<Recovery>,
}

#[derive(Debug)]
pub struct Workspace {
    tabs: Vec<Tab>,
    active: usize,
    /// Paths of closed tabs, most recent last.
    closed: Vec<PathBuf>,
    store: Option<RecoveryStore>,
    next_id: TabId,
}

const CLOSED_HISTORY: usize = 20;

impl Workspace {
    /// An empty workspace with one untitled tab.
    pub fn new(store: Option<RecoveryStore>) -> Self {
        let mut ws = Self {
            tabs: Vec::new(),
            active: 0,
            closed: Vec::new(),
            store,
            next_id: 1,
        };
        ws.push(Editor::new(), None);
        ws
    }

    /// Restore every snapshot as a tab, then open `paths` (skipping files
    /// already restored). Returns load errors for paths that failed and the
    /// number of restored tabs. The initial untitled tab is replaced.
    pub fn startup(
        store: Option<RecoveryStore>,
        snapshots: Vec<Entry>,
        paths: &[PathBuf],
    ) -> (Self, Vec<(PathBuf, LoadError)>, usize) {
        let mut ws = Self::new(store);
        let mut restored = 0;
        for entry in snapshots {
            let ed = Editor::restore(&entry.snapshot);
            if ed.document().is_dirty() {
                ws.add(ed, Some(entry.id));
                restored += 1;
            } else if let Some(s) = &ws.store {
                // Identical to disk: the checkpoint is stale.
                let _ = s.remove(&entry.id);
            }
        }
        let mut errors = Vec::new();
        for p in paths {
            if let Err(e) = ws.open(p) {
                errors.push((p.clone(), e));
            }
        }
        (ws, errors, restored)
    }

    fn push(&mut self, editor: Editor, recovery_id: Option<String>) -> usize {
        let recovery = self.store.clone().map(|s| Recovery::start(s, recovery_id));
        let id = self.next_id;
        self.next_id += 1;
        self.tabs.push(Tab {
            id,
            editor,
            recovery,
        });
        self.tabs.len() - 1
    }

    /// Add a tab and make it active. An untouched untitled tab in front is
    /// replaced, as there is nothing in it to keep.
    fn add(&mut self, editor: Editor, recovery_id: Option<String>) -> usize {
        if self.tabs.len() == 1 && self.is_blank(0) {
            self.tabs.clear();
        }
        self.active = self.push(editor, recovery_id);
        self.active
    }

    fn is_blank(&self, i: usize) -> bool {
        let ed = &self.tabs[i].editor;
        ed.file().is_none() && ed.document().len_chars() == 0 && !ed.document().can_undo()
    }

    pub fn new_tab(&mut self) -> usize {
        self.active = self.push(Editor::new(), None);
        self.active
    }

    /// Open `path` in a tab, or switch to it if it is already open.
    pub fn open(&mut self, path: &Path) -> Result<usize, LoadError> {
        if let Some(i) = self.find(path) {
            self.active = i;
            return Ok(i);
        }
        let ed = Editor::open(path)?;
        Ok(self.add(ed, None))
    }

    fn find(&self, path: &Path) -> Option<usize> {
        let same = |a: &Path| match (a.canonicalize(), path.canonicalize()) {
            (Ok(x), Ok(y)) => x == y,
            _ => a == path,
        };
        self.tabs
            .iter()
            .position(|t| t.editor.file().is_some_and(|f| same(&f.path)))
    }

    /// Close tab `i` without asking: the caller has already resolved
    /// unsaved changes. Its checkpoint is removed. The window always keeps
    /// at least one tab.
    pub fn close(&mut self, i: usize) {
        if i >= self.tabs.len() {
            return;
        }
        let mut tab = self.tabs.remove(i);
        if let Some(r) = &mut tab.recovery {
            r.discard();
        }
        if let Some(f) = tab.editor.file() {
            self.closed.retain(|p| p != &f.path);
            self.closed.push(f.path.clone());
            if self.closed.len() > CLOSED_HISTORY {
                self.closed.remove(0);
            }
        }
        if self.tabs.is_empty() {
            self.push(Editor::new(), None);
        }
        if self.active > i || self.active >= self.tabs.len() {
            self.active = self.active.saturating_sub(1);
        }
    }

    /// Reopen the most recently closed file that still loads.
    pub fn reopen_closed(&mut self) -> Option<Result<usize, LoadError>> {
        let path = self.closed.pop()?;
        Some(self.open(&path))
    }

    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    pub fn len(&self) -> usize {
        self.tabs.len()
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn active_index(&self) -> usize {
        self.active
    }

    pub fn active(&self) -> &Tab {
        &self.tabs[self.active]
    }

    pub fn active_mut(&mut self) -> &mut Tab {
        &mut self.tabs[self.active]
    }

    pub fn activate(&mut self, i: usize) {
        if i < self.tabs.len() {
            self.active = i;
        }
    }

    /// Cycle to the next (`forward`) or previous tab.
    pub fn cycle(&mut self, forward: bool) {
        let n = self.tabs.len();
        self.active = if forward {
            (self.active + 1) % n
        } else {
            (self.active + n - 1) % n
        };
    }

    /// Move tab `from` to position `to`, keeping the same tab active.
    pub fn move_tab(&mut self, from: usize, to: usize) {
        if from >= self.tabs.len() || to >= self.tabs.len() || from == to {
            return;
        }
        let active_id = self.tabs[self.active].id;
        let tab = self.tabs.remove(from);
        self.tabs.insert(to, tab);
        self.active = self.position(active_id).unwrap_or(0);
    }

    pub fn position(&self, id: TabId) -> Option<usize> {
        self.tabs.iter().position(|t| t.id == id)
    }

    pub fn first_dirty(&self) -> Option<usize> {
        self.tabs
            .iter()
            .position(|t| t.editor.document().is_dirty())
    }

    /// Advance every tab's checkpoints. Returns the soonest due time.
    /// The first recovery error, if any, is returned for display.
    pub fn tick(&mut self, now: Instant) -> (Option<Duration>, Option<String>) {
        let mut soonest: Option<Duration> = None;
        let mut error = None;
        for tab in &mut self.tabs {
            if let Some(r) = &mut tab.recovery {
                if let Some(d) = r.tick(&tab.editor, now) {
                    soonest = Some(soonest.map_or(d, |s| s.min(d)));
                }
                error = error.or_else(|| r.take_error());
            }
        }
        (soonest, error)
    }

    /// Wait for pending checkpoint I/O; call before exiting.
    pub fn flush(&self) {
        for tab in &self.tabs {
            if let Some(r) = &tab.recovery {
                r.flush(Duration::from_secs(2));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Command;
    use std::fs;

    #[test]
    fn open_replaces_blank_tab_and_dedupes() {
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a.txt");
        let b = dir.path().join("b.txt");
        fs::write(&a, "A").unwrap();
        fs::write(&b, "B").unwrap();
        let mut ws = Workspace::new(None);
        ws.open(&a).unwrap();
        assert_eq!(ws.len(), 1, "blank tab replaced");
        ws.open(&b).unwrap();
        assert_eq!(ws.len(), 2);
        assert_eq!(ws.open(&a).unwrap(), 0, "already open: switch to it");
        assert_eq!(ws.active_index(), 0);
    }

    #[test]
    fn close_keeps_one_tab_and_tracks_history() {
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a.txt");
        fs::write(&a, "A").unwrap();
        let mut ws = Workspace::new(None);
        ws.open(&a).unwrap();
        ws.new_tab();
        ws.activate(0);
        ws.close(0);
        assert_eq!(ws.len(), 1);
        assert_eq!(ws.active_index(), 0);
        ws.close(0);
        assert_eq!(ws.len(), 1, "never zero tabs");
        let i = ws.reopen_closed().unwrap().unwrap();
        assert_eq!(ws.tabs()[i].editor.document().text(), "A");
        assert!(ws.reopen_closed().is_none());
    }

    #[test]
    fn cycle_move_and_active_tracking() {
        let mut ws = Workspace::new(None);
        ws.active_mut().editor.execute(Command::Insert("0".into()));
        ws.new_tab();
        ws.new_tab();
        assert_eq!(ws.active_index(), 2);
        ws.cycle(true);
        assert_eq!(ws.active_index(), 0);
        ws.cycle(false);
        assert_eq!(ws.active_index(), 2);
        let id = ws.active().id;
        ws.move_tab(2, 0);
        assert_eq!(ws.active().id, id);
        assert_eq!(ws.active_index(), 0);
        assert_eq!(ws.first_dirty(), Some(1));
    }

    #[test]
    fn startup_restores_every_snapshot_as_a_tab() {
        use sanain_io::TextEncoding;
        use sanain_io::recovery::Snapshot;
        let dir = tempfile::tempdir().unwrap();
        let store = RecoveryStore::open(dir.path().join("r")).unwrap();
        for (id, text) in [("one", "first"), ("two", "second")] {
            let s = Snapshot {
                path: None,
                encoding: TextEncoding::Utf8,
                text: text.into(),
            };
            store.write(id, &s).unwrap();
        }
        let entries = store
            .list()
            .unwrap()
            .into_iter()
            .filter_map(Result::ok)
            .collect();
        let (ws, errors, restored) = Workspace::startup(Some(store), entries, &[]);
        assert!(errors.is_empty());
        assert_eq!(restored, 2);
        assert_eq!(ws.len(), 2);
        let mut texts: Vec<_> = ws
            .tabs()
            .iter()
            .map(|t| t.editor.document().text())
            .collect();
        texts.sort();
        assert_eq!(texts, ["first", "second"]);
    }
}
