//! Open documents in one window: tabs, the active tab, recently closed
//! files and each tab's crash-recovery checkpoints. No GUI types.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use text3_io::LoadError;
use text3_io::recovery::{Entry, RecoveryStore};

use crate::Editor;
use crate::editor::Restored;
use crate::panes::Panes;
use crate::recovery::{KeepError, Recovery};
use crate::session::{Session, SessionTab};

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

impl Tab {
    /// The id of this tab's recovery snapshot (`None` without a store).
    pub fn recovery_id(&self) -> Option<&str> {
        self.recovery.as_ref().map(Recovery::id)
    }
}

/// What [`Workspace::hot_exit`] did with the tabs that have unsaved changes.
#[derive(Debug, Default)]
pub struct HotExit {
    /// Written to the recovery store: they come back at the next start.
    pub kept: Vec<TabId>,
    /// Not written; the user must be asked about these.
    pub failed: Vec<(TabId, KeepError)>,
}

#[derive(Debug)]
pub struct Workspace {
    tabs: Vec<Tab>,
    active: usize,
    /// Paths of closed tabs, most recent last.
    closed: Vec<PathBuf>,
    store: Option<RecoveryStore>,
    next_id: TabId,
    /// Restored tabs whose file changed or vanished (see [`Restored`]).
    restore_notes: Vec<(TabId, Restored)>,
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
            restore_notes: Vec::new(),
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
            let (ed, state) = Editor::restore_checked(&entry.snapshot);
            if ed.document().is_dirty() {
                let i = ws.add(ed, Some(entry.id));
                if state != Restored::Applied {
                    ws.restore_notes.push((ws.tabs[i].id, state));
                }
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

    /// Restored tabs whose file changed on disk or is missing, once.
    pub fn take_restore_notes(&mut self) -> Vec<(TabId, Restored)> {
        std::mem::take(&mut self.restore_notes)
    }

    /// Whether unsaved text can be kept in a recovery store at all.
    pub fn has_store(&self) -> bool {
        self.store.is_some()
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

    /// Write every tab with unsaved changes to the recovery store now and
    /// wait for each write (hot exit), giving up on a tab after `limit`.
    /// Texts over `max_len` bytes are not written. Nothing is closed or
    /// removed: kept tabs stay as they are until the process exits.
    pub fn hot_exit(&mut self, max_len: usize, limit: Duration) -> HotExit {
        let mut out = HotExit::default();
        for tab in &mut self.tabs {
            if !tab.editor.document().is_dirty() {
                continue;
            }
            let result = match &mut tab.recovery {
                Some(r) => r.write_now(&tab.editor, max_len, limit),
                None => Err(KeepError::Failed("recovery is off".into())),
            };
            match result {
                Ok(()) => out.kept.push(tab.id),
                Err(e) => out.failed.push((tab.id, e)),
            }
        }
        out
    }

    /// Files open in this window, in tab order, and the split layout (if
    /// any), for session restore. Untitled tabs are left out, as are panes
    /// showing them.
    pub fn session(&self, split: Option<&Panes<TabId>>) -> Session {
        self.session_with(split, &[])
    }

    /// [`Self::session`], plus the tabs in `kept` (written by hot exit)
    /// with their recovery ids, untitled ones included.
    pub fn session_with(&self, split: Option<&Panes<TabId>>, kept: &[TabId]) -> Session {
        let mut tabs = Vec::new();
        let mut pinned = Vec::new();
        let mut index = HashMap::new();
        let mut active = 0;
        for (i, t) in self.tabs.iter().enumerate() {
            let recovery = kept
                .contains(&t.id)
                .then(|| t.recovery_id().map(str::to_owned))
                .flatten();
            let path = t.editor.file().map(|f| f.path.clone());
            if path.is_none() && recovery.is_none() {
                continue;
            }
            if i == self.active {
                active = tabs.len();
            }
            if t.pinned {
                pinned.push(tabs.len());
            }
            index.insert(t.id, tabs.len());
            let sel = t.editor.selections().primary();
            tabs.push(SessionTab {
                path,
                recovery,
                caret: Some((sel.anchor, sel.head)).filter(|c| *c != (0, 0)),
            });
        }
        let layout = split
            .filter(|s| s.is_split())
            .map(|s| s.layout().filter_map(|id| index.get(&id).copied()))
            .filter(|l| l.len() >= 2);
        Session {
            tabs,
            active,
            pinned,
            layout,
        }
    }

    /// Reopen a saved session. Tabs kept by hot exit are matched to the
    /// tabs restored from their recovery snapshots; with `files`, other
    /// files are opened too (missing ones are skipped silently, open ones
    /// not duplicated). Session tabs are put first, in their saved order,
    /// with their saved selection; other restored tabs follow. Returns the
    /// split layout, without panes whose tab did not come back; its
    /// focused pane's tab becomes the active one.
    pub fn restore_session(&mut self, s: &Session, files: bool) -> Option<Panes<TabId>> {
        let mut active = None;
        let mut pin = Vec::new();
        let mut order = Vec::new();
        let mut ids = vec![None; s.tabs.len()];
        for (i, entry) in s.tabs.iter().enumerate() {
            let kept = entry
                .recovery
                .as_deref()
                .and_then(|rid| self.tabs.iter().position(|t| t.recovery_id() == Some(rid)));
            let idx = match (kept, &entry.path) {
                (Some(idx), _) => Some(idx),
                (None, Some(p)) if files => self.open(p).ok(),
                _ => None,
            };
            let Some(idx) = idx else { continue };
            let id = self.tabs[idx].id;
            if order.contains(&id) {
                continue;
            }
            if let Some((a, h)) = entry.caret {
                self.tabs[idx].editor.set_selection(a, h);
            }
            order.push(id);
            ids[i] = Some(id);
            if i == s.active {
                active = Some(id);
            }
            if s.pinned.contains(&i) {
                pin.push(id);
            }
        }
        self.reorder(&order);
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

    /// Put the tabs `first` at the front in that order; the rest keep
    /// their order after them. The active tab stays active.
    fn reorder(&mut self, first: &[TabId]) {
        let active_id = self.tabs[self.active].id;
        let mut rest = std::mem::take(&mut self.tabs);
        for id in first {
            if let Some(p) = rest.iter().position(|t| t.id == *id) {
                self.tabs.push(rest.remove(p));
            }
        }
        self.tabs.extend(rest);
        self.active = self.position(active_id).unwrap_or(0);
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
        assert_eq!(s.paths(), vec![a.clone(), b.clone()]);
        assert_eq!(s.tabs.len(), 2);
        assert_eq!(s.active, 1);
        fs::remove_file(&a).unwrap();
        let mut ws2 = Workspace::new(None);
        assert_eq!(ws2.restore_session(&s, true), None);
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
        let back = ws2.restore_session(&s, true).unwrap();
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
        let back = ws3.restore_session(&s, true).unwrap();
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
                base: None,
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

    fn snapshots(store: &RecoveryStore) -> Vec<Entry> {
        store
            .list()
            .unwrap()
            .into_iter()
            .map(|r| r.unwrap())
            .collect()
    }

    const WAIT: Duration = Duration::from_secs(10);

    /// Quit with hot exit, then start again: untitled and edited tabs come
    /// back with their text, still unsaved, in order, with the selection,
    /// pins and split; the file keeps its encoding and line endings.
    #[test]
    fn hot_exit_then_restart_restores_every_tab() {
        let dir = tempfile::tempdir().unwrap();
        let store = RecoveryStore::open(dir.path().join("r")).unwrap();
        let utf16 = dir.path().join("wide.txt");
        let mut bytes = vec![0xFF, 0xFE];
        bytes.extend("one\r\ntwo\r\n".encode_utf16().flat_map(u16::to_le_bytes));
        fs::write(&utf16, &bytes).unwrap();
        let clean = dir.path().join("clean.txt");
        fs::write(&clean, "as is\n").unwrap();

        let mut ws = Workspace::new(Some(store.clone()));
        ws.active_mut()
            .editor
            .execute(Command::Insert("scratch\nnote".into()));
        let untitled = ws.active().id;
        ws.active_mut().editor.set_selection(2, 5);
        let i = ws.open(&utf16).unwrap();
        let wide = ws.tabs()[i].id;
        ws.tab_mut(i).unwrap().editor.select(5..5);
        ws.tab_mut(i)
            .unwrap()
            .editor
            .execute(Command::Insert("2½".into()));
        ws.open(&clean).unwrap();
        ws.new_tab(); // empty: nothing to keep, not recorded
        ws.set_pinned(ws.position(wide).unwrap(), true);
        ws.activate(ws.position(untitled).unwrap());
        let mut split = Panes::new(untitled);
        split.split(Direction::Right, wide);

        let kept = ws.hot_exit(usize::MAX, WAIT);
        assert!(kept.failed.is_empty(), "{:?}", kept.failed);
        assert_eq!(kept.kept.len(), 2);
        let session = ws.session_with(Some(&split), &kept.kept);
        drop(ws); // the process ends: nothing is closed or removed

        let session = Session::decode(&session.encode()).unwrap();
        let (mut ws, errors, restored) =
            Workspace::startup(Some(store.clone()), snapshots(&store), &[]);
        assert!(errors.is_empty());
        assert_eq!(restored, 2);
        let back = ws.restore_session(&session, true).unwrap();
        assert_eq!(back.len(), 2);
        let names: Vec<_> = ws.tabs().iter().map(|t| t.editor.file_name()).collect();
        assert_eq!(
            names,
            [Some("wide.txt".into()), None, Some("clean.txt".into())],
            "pinned first, then the saved order"
        );
        let [w, u, c] = [0, 1, 2].map(|i| &ws.tabs()[i]);
        assert!(w.pinned);
        assert_eq!(u.editor.document().text(), "scratch\nnote");
        assert!(u.editor.document().is_dirty());
        assert!(u.editor.file().is_none());
        let sel = u.editor.selections().primary();
        assert_eq!((sel.anchor, sel.head), (2, 5));
        assert_eq!(w.editor.document().text(), "one\r\n2½two\r\n");
        assert!(w.editor.document().is_dirty());
        assert_eq!(w.editor.encoding(), text3_io::TextEncoding::Utf16LeBom);
        assert!(!c.editor.document().is_dirty());
        assert!(
            ws.active().editor.file().is_none(),
            "the focused pane's tab"
        );
        assert!(ws.take_restore_notes().is_empty());

        // Saving writes the restored edit in the file's own encoding.
        let i = ws.position(ws.tabs()[0].id).unwrap();
        assert!(matches!(
            ws.tab_mut(i).unwrap().editor.save(),
            crate::SaveOutcome::Saved
        ));
        let mut want = vec![0xFF, 0xFE];
        want.extend("one\r\n2½two\r\n".encode_utf16().flat_map(u16::to_le_bytes));
        assert_eq!(fs::read(&utf16).unwrap(), want);
    }

    #[test]
    fn without_session_restore_only_kept_tabs_come_back() {
        let dir = tempfile::tempdir().unwrap();
        let store = RecoveryStore::open(dir.path().join("r")).unwrap();
        let clean = dir.path().join("clean.txt");
        fs::write(&clean, "x").unwrap();
        let mut ws = Workspace::new(Some(store.clone()));
        ws.open(&clean).unwrap();
        ws.new_tab();
        ws.active_mut().editor.execute(Command::Insert("b".into()));
        ws.new_tab();
        ws.active_mut().editor.execute(Command::Insert("a".into()));
        ws.move_tab(2, 1); // order: clean, "a", "b"
        let kept = ws.hot_exit(usize::MAX, WAIT).kept;
        let session = ws.session_with(None, &kept);
        drop(ws);
        let (mut ws, _, _) = Workspace::startup(Some(store.clone()), snapshots(&store), &[]);
        ws.restore_session(&session, false);
        let texts: Vec<_> = ws
            .tabs()
            .iter()
            .map(|t| t.editor.document().text())
            .collect();
        assert_eq!(texts, ["a", "b"], "the clean file is not reopened");
    }

    #[test]
    fn many_tabs_are_all_kept_in_order() {
        let dir = tempfile::tempdir().unwrap();
        let store = RecoveryStore::open(dir.path().join("r")).unwrap();
        let mut ws = Workspace::new(Some(store.clone()));
        for n in 0..60 {
            if n > 0 {
                ws.new_tab();
            }
            ws.active_mut()
                .editor
                .execute(Command::Insert(format!("tab {n}")));
        }
        let kept = ws.hot_exit(usize::MAX, WAIT).kept;
        assert_eq!(kept.len(), 60);
        let session = ws.session_with(None, &kept);
        drop(ws);
        let (mut ws, _, restored) = Workspace::startup(Some(store.clone()), snapshots(&store), &[]);
        assert_eq!(restored, 60);
        ws.restore_session(&session, true);
        let texts: Vec<_> = ws
            .tabs()
            .iter()
            .map(|t| t.editor.document().text())
            .collect();
        let want: Vec<_> = (0..60).map(|n| format!("tab {n}")).collect();
        assert_eq!(texts, want);
    }

    /// A tab over the size cap is not written (the others are) and is
    /// left for the prompt; it is not recorded as kept.
    #[test]
    fn a_tab_over_the_cap_is_left_to_the_prompt() {
        let dir = tempfile::tempdir().unwrap();
        let store = RecoveryStore::open(dir.path().join("r")).unwrap();
        let mut ws = Workspace::new(Some(store.clone()));
        ws.active_mut()
            .editor
            .execute(Command::Insert("small".into()));
        ws.new_tab();
        ws.active_mut()
            .editor
            .execute(Command::Insert("x".repeat(100)));
        let big = ws.active().id;
        let r = ws.hot_exit(64, WAIT);
        assert_eq!(r.kept.len(), 1);
        assert_eq!(r.failed, vec![(big, KeepError::TooLarge)]);
        let session = ws.session_with(None, &r.kept);
        assert_eq!(session.tabs.len(), 1);
        let texts: Vec<_> = snapshots(&store)
            .into_iter()
            .map(|e| e.snapshot.text)
            .collect();
        assert_eq!(texts, ["small"]);
    }

    #[test]
    fn without_a_store_every_unsaved_tab_is_left_to_the_prompt() {
        let mut ws = Workspace::new(None);
        assert!(!ws.has_store());
        ws.active_mut().editor.execute(Command::Insert("x".into()));
        ws.new_tab(); // clean: not asked about
        let r = ws.hot_exit(usize::MAX, WAIT);
        assert!(r.kept.is_empty());
        assert_eq!(r.failed.len(), 1);
    }

    /// Another instance's kept tabs are not in this session file (the last
    /// instance to quit writes it) but still come back, after the
    /// session's tabs.
    #[test]
    fn snapshots_missing_from_the_session_still_come_back() {
        let dir = tempfile::tempdir().unwrap();
        let store = RecoveryStore::open(dir.path().join("r")).unwrap();
        let mut first = Workspace::new(Some(store.clone()));
        first
            .active_mut()
            .editor
            .execute(Command::Insert("first".into()));
        let mut second = Workspace::new(Some(store.clone()));
        second
            .active_mut()
            .editor
            .execute(Command::Insert("second".into()));
        let a = first.hot_exit(usize::MAX, WAIT).kept;
        first.session_with(None, &a); // written, then overwritten:
        let b = second.hot_exit(usize::MAX, WAIT).kept;
        let session = second.session_with(None, &b);
        drop((first, second));
        let (mut ws, _, _) = Workspace::startup(Some(store.clone()), snapshots(&store), &[]);
        ws.restore_session(&session, true);
        let texts: Vec<_> = ws
            .tabs()
            .iter()
            .map(|t| t.editor.document().text())
            .collect();
        assert_eq!(texts, ["second", "first"]);
    }

    #[test]
    fn restore_notes_name_changed_and_missing_files() {
        let dir = tempfile::tempdir().unwrap();
        let store = RecoveryStore::open(dir.path().join("r")).unwrap();
        let [changed, gone] = ["changed.txt", "gone.txt"].map(|n| dir.path().join(n));
        fs::write(&changed, "base\n").unwrap();
        fs::write(&gone, "base\n").unwrap();
        let mut ws = Workspace::new(Some(store.clone()));
        for p in [&changed, &gone] {
            let i = ws.open(p).unwrap();
            ws.tab_mut(i)
                .unwrap()
                .editor
                .execute(Command::Insert("mine ".into()));
        }
        let kept = ws.hot_exit(usize::MAX, WAIT).kept;
        let session = ws.session_with(None, &kept);
        drop(ws);
        fs::write(&changed, "theirs\n").unwrap();
        fs::remove_file(&gone).unwrap();
        let (mut ws, _, restored) = Workspace::startup(Some(store.clone()), snapshots(&store), &[]);
        assert_eq!(restored, 2);
        ws.restore_session(&session, true);
        assert_eq!(ws.len(), 2, "the missing file is not dropped or duplicated");
        let mut notes: Vec<_> = ws
            .take_restore_notes()
            .into_iter()
            .map(|(id, state)| {
                let t = &ws.tabs()[ws.position(id).unwrap()];
                (
                    t.editor.file_name().unwrap(),
                    t.editor.document().text(),
                    state,
                )
            })
            .collect();
        notes.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(
            notes,
            [
                (
                    "changed.txt".into(),
                    "mine base\n".into(),
                    Restored::ChangedOnDisk
                ),
                ("gone.txt".into(), "mine base\n".into(), Restored::Missing),
            ]
        );
        assert_eq!(fs::read(&changed).unwrap(), b"theirs\n", "not overwritten");
        assert!(!gone.exists(), "not recreated");
    }
}
