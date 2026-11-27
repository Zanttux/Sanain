//! Open documents in one window: tabs, the active tab, recently closed
//! files and each tab's crash-recovery checkpoints. No GUI types.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use text3_io::LoadError;
use text3_io::recovery::{Entry, RecoveryStore};

use crate::Editor;
use crate::panes::Panes;
use crate::recovery::Recovery;

/// Stable identity of a tab for the lifetime of the process.
pub type TabId = u64;

#[derive(Debug)]
pub struct Tab {
    pub id: TabId,
    pub editor: Editor,
    /// Pinned tabs stay together at the front of the tab bar.
    pub pinned: bool,
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
            pinned: false,
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

    /// (Re)open `path` with a chosen encoding. An existing tab for the
    /// file is replaced in place (the caller ensures it has no unsaved
    /// changes); otherwise a new tab is added.
    pub fn open_as(
        &mut self,
        path: &Path,
        encoding: text3_io::TextEncoding,
    ) -> Result<usize, LoadError> {
        let ed = Editor::open_as(path, encoding)?;
        match self.find(path) {
            Some(i) => {
                self.tabs[i].editor = ed;
                self.active = i;
                Ok(i)
            }
            None => Ok(self.add(ed, None)),
        }
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

    pub fn tab_mut(&mut self, i: usize) -> Option<&mut Tab> {
        self.tabs.get_mut(i)
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

    /// Number of pinned tabs (always the first ones).
    pub fn pinned_count(&self) -> usize {
        self.tabs.iter().take_while(|t| t.pinned).count()
    }

    /// Pin or unpin tab `i`, moving it to the end of the pinned group or
    /// the start of the others.
    pub fn set_pinned(&mut self, i: usize, pinned: bool) {
        let Some(tab) = self.tabs.get(i) else { return };
        if tab.pinned == pinned {
            return;
        }
        let others = self
            .tabs
            .iter()
            .enumerate()
            .filter(|(j, t)| *j != i && t.pinned)
            .count();
        let active_id = self.tabs[self.active].id;
        let mut tab = self.tabs.remove(i);
        tab.pinned = pinned;
        self.tabs.insert(others, tab);
        self.active = self.position(active_id).unwrap_or(0);
    }

    /// Move tab `from` to position `to`, keeping the same tab active.
    /// Tabs stay within their group: pinned ones before all others.
    pub fn move_tab(&mut self, from: usize, to: usize) {
        if from >= self.tabs.len() || to >= self.tabs.len() {
            return;
        }
        let pc = self.pinned_count();
        let to = if self.tabs[from].pinned {
            to.min(pc.saturating_sub(1))
        } else {
            to.max(pc)
        };
        if from == to {
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

    /// Files open in this window, in tab order, and the split layout (if
    /// any), for session restore. Panes showing an untitled tab are left
    /// out of the layout.
    pub fn session(&self, split: Option<&Panes<TabId>>) -> crate::session::Session {
        let mut paths = Vec::new();
        let mut pinned = Vec::new();
        let mut index = HashMap::new();
        let mut active = 0;
        for (i, t) in self.tabs.iter().enumerate() {
            if let Some(f) = t.editor.file() {
                if i == self.active {
                    active = paths.len();
                }
                if t.pinned {
                    pinned.push(paths.len());
                }
                index.insert(t.id, paths.len());
                paths.push(f.path.clone());
            }
        }
        let layout = split
            .filter(|s| s.is_split())
            .map(|s| s.layout().filter_map(|id| index.get(&id).copied()))
            .filter(|l| l.len() >= 2);
        crate::session::Session {
            paths,
            active,
            pinned,
            layout,
        }
    }

    /// Reopen a saved session: missing files are skipped silently, files
    /// already open (e.g. restored from recovery) are not duplicated.
    /// Returns the split layout, without panes whose file did not open;
    /// its focused pane's tab becomes the active one.
    pub fn restore_session(&mut self, s: &crate::session::Session) -> Option<Panes<TabId>> {
        let mut active = None;
        let mut pin = Vec::new();
        let mut ids = vec![None; s.paths.len()];
        for (i, p) in s.paths.iter().enumerate() {
            if let Ok(idx) = self.open(p) {
                ids[i] = Some(self.tabs[idx].id);
                if i == s.active {
                    active = Some(self.tabs[idx].id);
                }
                if s.pinned.contains(&i) {
                    pin.push(self.tabs[idx].id);
                }
            }
        }
        for id in pin {
            if let Some(i) = self.position(id) {
                self.set_pinned(i, true);
            }
        }
        if let Some(a) = active.and_then(|id| self.position(id)) {
            self.active = a;
        }
        let split = s
            .layout
            .clone()
            .map(|l| l.filter_map(|i| ids.get(i).copied().flatten()))
            .and_then(Panes::from_layout)?;
        if let Some(a) = self.position(split.focused().tab) {
            self.active = a;
        }
        Some(split)
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
    use crate::panes::{Direction, DividerId};
    use std::fs;

    /// Reopening a tab in another encoding replaces its document; caches
    /// keyed by revision (structured views, highlighting) must not see the
    /// new text as the one they were built from (CSV edit offsets from the
    /// old text corrupted the new one).
    #[test]
    fn reopen_with_encoding_gets_a_new_revision() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("t.csv");
        fs::write(&p, "name,city\n\u{c4}iti,Espoo\nBob,Turku\n").unwrap();
        let mut ws = Workspace::new(None);
        let i = ws
            .open_as(
                &p,
                text3_io::TextEncoding::Legacy(encoding_rs::WINDOWS_1252),
            )
            .unwrap();
        let before = ws.tabs()[i].editor.document().revision();
        let i = ws.open_as(&p, text3_io::TextEncoding::Utf8).unwrap();
        let after = ws.tabs()[i].editor.document().revision();
        assert_ne!(before, after);
        assert!(ws.tabs()[i].editor.document().text().contains("\u{c4}iti"));
    }

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
    fn session_round_trip_skips_missing_and_untitled() {
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a.txt");
        let b = dir.path().join("b.txt");
        fs::write(&a, "A").unwrap();
        fs::write(&b, "B").unwrap();
        let mut ws = Workspace::new(None);
        ws.open(&a).unwrap();
        ws.new_tab(); // untitled: not recorded
        ws.open(&b).unwrap();
        let s = ws.session(None);
        assert_eq!(s.paths, vec![a.clone(), b.clone()]);
        assert_eq!(s.active, 1);
        fs::remove_file(&a).unwrap();
        let mut ws2 = Workspace::new(None);
        assert_eq!(ws2.restore_session(&s), None);
        assert_eq!(ws2.len(), 1);
        assert_eq!(ws2.active().editor.document().text(), "B");
    }

    #[test]
    fn split_layout_survives_a_session_and_drops_missing_files() {
        let dir = tempfile::tempdir().unwrap();
        let [a, b, c] = ["a.txt", "b.txt", "c.txt"].map(|n| dir.path().join(n));
        for p in [&a, &b, &c] {
            fs::write(p, "x").unwrap();
        }
        let mut ws = Workspace::new(None);
        let ids = [&a, &b, &c].map(|p| {
            let i = ws.open(p).unwrap();
            ws.tabs()[i].id
        });
        let untitled = {
            ws.new_tab();
            ws.active().id
        };
        let mut split = Panes::new(ids[0]);
        split.split(Direction::Right, ids[1]);
        split.set_focus(1);
        split.split(Direction::Down, ids[2]);
        split.move_divider(DividerId::Outer(0), 0.3, 0.1);
        split.set_focus(2);
        let s = ws.session(Some(&split));
        let mut ws2 = Workspace::new(None);
        let back = ws2.restore_session(&s).unwrap();
        assert_eq!(back.len(), 3);
        assert_eq!(back.direction(), Direction::Right);
        assert!((back.fractions()[0].x1 - 0.3).abs() < 1e-5);
        let shown: Vec<_> = back.panes().map(|p| ws2.position(p.tab)).collect();
        assert_eq!(shown, [Some(0), Some(1), Some(2)]);
        // The focused pane's tab is the active one.
        assert_eq!(ws2.active().id, back.focused().tab);
        assert_eq!(ws2.active_index(), 2);

        // A file that is gone takes its pane with it.
        fs::remove_file(&b).unwrap();
        let mut ws3 = Workspace::new(None);
        let back = ws3.restore_session(&s).unwrap();
        assert_eq!(back.len(), 2);
        assert_eq!(back.direction(), Direction::Right);
        // Only the file in a pane with an untitled tab: no split.
        split.set_tab(1, untitled);
        split.set_tab(2, untitled);
        assert_eq!(ws.session(Some(&split)).layout, None);
    }

    #[test]
    fn startup_restores_every_snapshot_as_a_tab() {
        use text3_io::TextEncoding;
        use text3_io::recovery::Snapshot;
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

    #[test]
    fn pinned_tabs_stay_in_front() {
        let mut ws = Workspace::new(None);
        for _ in 0..3 {
            ws.new_tab();
        }
        // Four tabs: ids 0..=3, last one active.
        let base = ws.tabs()[0].id;
        let ids = |ws: &Workspace| ws.tabs().iter().map(|t| t.id - base).collect::<Vec<_>>();
        assert_eq!(ids(&ws), vec![0, 1, 2, 3]);
        ws.set_pinned(2, true);
        assert_eq!(ids(&ws), vec![2, 0, 1, 3]);
        ws.set_pinned(3, true);
        assert_eq!(ids(&ws), vec![2, 3, 0, 1]);
        assert_eq!(ws.pinned_count(), 2);
        assert_eq!(ws.active().id - base, 3, "active tab follows its move");
        ws.move_tab(3, 0);
        assert_eq!(ids(&ws), vec![2, 3, 1, 0], "unpinned stays after pinned");
        ws.move_tab(0, 3);
        assert_eq!(ids(&ws), vec![3, 2, 1, 0], "pinned stays in its group");
        ws.set_pinned(0, false);
        assert_eq!(ids(&ws), vec![2, 3, 1, 0]);
        assert_eq!(ws.pinned_count(), 1);
    }
}
