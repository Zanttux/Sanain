//! One open document plus its view state and file binding. No GUI types.

use std::path::{Path, PathBuf};

use sanain_core::folding::{self, FoldMap};
use sanain_core::lines;
use sanain_core::selection::{self as sel, Selection, Selections};
use sanain_core::{Document, LineEnding, Transaction};
use sanain_io::{Fingerprint, LoadError, SaveError, SaveMode, TextEncoding};

/// Editing commands a view can send. Text input arrives already composed
/// (IME commit, paste, typed characters).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Insert(String),
    NewLine,
    Backspace,
    Delete,
    Left {
        extend: bool,
    },
    WordLeft {
        extend: bool,
    },
    WordRight {
        extend: bool,
    },
    DeleteWordBackward,
    DeleteWordForward,
    DeleteLine,
    DuplicateLine,
    MoveLineUp,
    MoveLineDown,
    Right {
        extend: bool,
    },
    Up {
        extend: bool,
    },
    Down {
        extend: bool,
    },
    LineStart {
        extend: bool,
    },
    LineEnd {
        extend: bool,
    },
    SelectAll,
    /// Place a single caret, or extend the primary selection to `at`.
    PlaceCaret {
        at: usize,
        extend: bool,
    },
    Undo,
    Redo,
    /// Remove the selection; returns its text for the clipboard.
    Cut,
}

#[derive(Debug, Clone)]
pub struct FileBinding {
    pub path: PathBuf,
    pub encoding: TextEncoding,
    fingerprint: Fingerprint,
}

/// Result of a save attempt the UI must react to.
#[derive(Debug)]
pub enum SaveOutcome {
    Saved,
    /// No path yet: ask the user for one, then call `save_as`.
    NeedsPath,
    /// The file changed on disk. Ask; on confirmation call `save_overwrite`.
    Conflict,
    ReadOnly,
    Failed(SaveError),
}

#[derive(Debug)]
pub struct Editor {
    doc: Document,
    sels: Selections,
    file: Option<FileBinding>,
    /// Encoding used when saving a buffer that has no file yet.
    untitled_encoding: TextEncoding,
    newline: &'static str,
    /// Goal columns kept across consecutive Up/Down moves.
    goal: Option<Vec<usize>>,
    /// Folded regions, as the char index where each fold's start line
    /// begins. Mapped through edits so folds stay on their lines.
    folds: Vec<usize>,
    /// Tab width the folds were computed with (from settings).
    fold_tab: usize,
    /// Cached hidden-line map: (revision, folds, tab width, map).
    fold_cache: Option<(sanain_core::Revision, Vec<usize>, usize, FoldMap)>,
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

impl Editor {
    pub fn new() -> Self {
        Self {
            doc: Document::new(),
            sels: Selections::single(Selection::cursor(0)),
            file: None,
            untitled_encoding: TextEncoding::Utf8,
            newline: platform_newline(),
            goal: None,
            folds: Vec::new(),
            fold_tab: 4,
            fold_cache: None,
        }
    }

    /// Rebuild a buffer from a recovery snapshot. If its file still loads,
    /// the recovered text is applied as one undoable edit on top of the disk
    /// version, so conflict detection and Undo both work. Otherwise the text
    /// becomes an untitled buffer.
    pub fn restore(snapshot: &sanain_io::recovery::Snapshot) -> Self {
        let mut ed = snapshot
            .path
            .as_deref()
            .and_then(|p| Self::open(p).ok())
            .unwrap_or_default();
        ed.untitled_encoding = snapshot.encoding;
        if ed.document().text() != snapshot.text {
            ed.execute(Command::SelectAll);
            ed.execute(Command::Insert(snapshot.text.clone()));
            ed.newline = preferred_newline(&snapshot.text);
        }
        ed.execute(Command::PlaceCaret {
            at: 0,
            extend: false,
        });
        ed
    }

    pub fn open(path: &Path) -> Result<Self, LoadError> {
        Self::from_loaded(path, sanain_io::load(path)?)
    }

    /// Open with an encoding the user chose.
    pub fn open_as(path: &Path, encoding: TextEncoding) -> Result<Self, LoadError> {
        Self::from_loaded(path, sanain_io::load_as(path, encoding)?)
    }

    fn from_loaded(path: &Path, loaded: sanain_io::Loaded) -> Result<Self, LoadError> {
        Ok(Self {
            newline: preferred_newline(&loaded.text),
            goal: None,
            folds: Vec::new(),
            fold_tab: 4,
            fold_cache: None,
            doc: Document::from_saved_text(&loaded.text),
            sels: Selections::single(Selection::cursor(0)),
            untitled_encoding: loaded.encoding,
            file: Some(FileBinding {
                path: path.to_owned(),
                encoding: loaded.encoding,
                fingerprint: loaded.fingerprint,
            }),
        })
    }

    pub fn document(&self) -> &Document {
        &self.doc
    }

    pub fn selections(&self) -> &Selections {
        &self.sels
    }

    pub fn file(&self) -> Option<&FileBinding> {
        self.file.as_ref()
    }

    pub fn encoding(&self) -> TextEncoding {
        self.file
            .as_ref()
            .map_or(self.untitled_encoding, |f| f.encoding)
    }

    pub fn line_ending(&self) -> LineEnding {
        match LineEnding::detect(&self.doc.text()) {
            LineEnding::None if self.newline == "\r\n" => LineEnding::CrLf,
            LineEnding::None => LineEnding::Lf,
            other => other,
        }
    }

    /// File name for titles and dialogs; `None` for an untitled buffer.
    pub fn file_name(&self) -> Option<String> {
        self.file
            .as_ref()
            .and_then(|f| f.path.file_name())
            .map(|n| n.to_string_lossy().into_owned())
    }

    /// The line break to insert: the document's first existing break, so
    /// edits never mix conventions; the file's or platform's default when
    /// the document has none.
    fn newline(&self) -> &'static str {
        let rope = self.doc.rope();
        if rope.len_lines() < 2 {
            return self.newline;
        }
        let end = sel::line_content_end(rope, 0);
        let brk: String = rope.slice(end..rope.line_to_char(1)).into();
        match brk.as_str() {
            "\r\n" => "\r\n",
            "\r" => "\r",
            "\n" => "\n",
            _ => self.newline,
        }
    }

    /// Run a command. Returns text for the clipboard (Cut) if any.
    pub fn execute(&mut self, cmd: Command) -> Option<String> {
        let goal = self.goal.take();
        let rope = self.doc.rope();
        let (tx, next): (Option<Transaction>, Selections) = match cmd {
            Command::Insert(text) => {
                let (tx, s) = sel::insert(&self.sels, &text);
                (Some(tx), s)
            }
            Command::NewLine => {
                let (tx, s) = sel::insert(&self.sels, self.newline());
                (Some(tx), s)
            }
            Command::Backspace => {
                let (tx, s) = sel::delete_backward(rope, &self.sels);
                (Some(tx), s)
            }
            Command::Delete => {
                let (tx, s) = sel::delete_forward(rope, &self.sels);
                (Some(tx), s)
            }
            Command::Cut => {
                let text = sel::selected_text(rope, &self.sels);
                if text.is_empty() {
                    return None;
                }
                let (tx, s) = sel::insert(&self.sels, "");
                self.commit(tx, s);
                return Some(text);
            }
            Command::WordLeft { extend } => (None, sel::move_word(rope, &self.sels, false, extend)),
            Command::WordRight { extend } => (None, sel::move_word(rope, &self.sels, true, extend)),
            Command::DeleteWordBackward => {
                let (tx, s) = sel::delete_word(rope, &self.sels, false);
                (Some(tx), s)
            }
            Command::DeleteWordForward => {
                let (tx, s) = sel::delete_word(rope, &self.sels, true);
                (Some(tx), s)
            }
            Command::Left { extend } => {
                (None, sel::move_horizontal(rope, &self.sels, false, extend))
            }
            Command::Right { extend } => {
                (None, sel::move_horizontal(rope, &self.sels, true, extend))
            }
            Command::Up { extend } | Command::Down { extend } => {
                let goals = goal
                    .filter(|g| g.len() == self.sels.len())
                    .unwrap_or_else(|| sel::columns(rope, &self.sels));
                let down = matches!(cmd, Command::Down { .. });
                let next = sel::move_vertical_to(rope, &self.sels, down, extend, &goals);
                self.goal = Some(goals);
                (None, next)
            }
            Command::DeleteLine => {
                let (tx, s) = lines::delete_lines(rope, &self.sels);
                (Some(tx), s)
            }
            Command::DuplicateLine => {
                let (tx, s) = lines::duplicate_lines(rope, &self.sels, self.newline());
                (Some(tx), s)
            }
            Command::MoveLineUp | Command::MoveLineDown => {
                let (tx, s) = lines::move_lines(rope, &self.sels, cmd == Command::MoveLineDown);
                (Some(tx), s)
            }
            Command::LineStart { extend } => {
                (None, sel::move_line_edge(rope, &self.sels, false, extend))
            }
            Command::LineEnd { extend } => {
                (None, sel::move_line_edge(rope, &self.sels, true, extend))
            }
            Command::SelectAll => (None, sel::select_all(rope)),
            Command::PlaceCaret { at, extend } => {
                let at = snap(rope, at);
                let s = if extend {
                    Selection::new(self.sels.primary().anchor, at)
                } else {
                    Selection::cursor(at)
                };
                (None, Selections::single(s))
            }
            Command::Undo | Command::Redo => {
                let carets = if cmd == Command::Undo {
                    self.doc.undo_carets()
                } else {
                    self.doc.redo_carets()
                };
                if let Some(c) = carets {
                    let len = self.doc.len_chars();
                    let list = c.into_iter().map(Selection::cursor).collect();
                    self.sels = Selections::new(list, 0, len);
                    // The undone change is not available to map folds
                    // through; open them rather than fold the wrong lines.
                    self.folds.clear();
                }
                return None;
            }
        };
        match tx {
            Some(tx) => self.commit(tx, next),
            None => self.sels = next,
        }
        self.unfold_around_caret();
        None
    }

    /// The primary selection's char range, or the whole document if the
    /// selection is empty: what a text tool acts on.
    pub fn tool_range(&self) -> std::ops::Range<usize> {
        let r = self.sels.primary().range();
        if r.is_empty() {
            0..self.doc.len_chars()
        } else {
            r
        }
    }

    /// Replace the tool range with `f(text)` as one undoable edit. Returns
    /// whether anything changed. A selection stays selected around the
    /// result; a whole-document change leaves the caret at the start.
    pub fn transform<E>(&mut self, f: impl FnOnce(&str) -> Result<String, E>) -> Result<bool, E> {
        let range = self.tool_range();
        let whole = range.start == 0 && range.end == self.doc.len_chars();
        let before: String = self.doc.rope().slice(range.clone()).into();
        let after = f(&before)?;
        if after == before {
            return Ok(false);
        }
        let len = after.chars().count();
        self.select(range.clone());
        self.execute(Command::Insert(after));
        if whole {
            self.select(0..0);
        } else {
            self.select(range.start..range.start + len);
        }
        Ok(true)
    }

    /// Replace a char range with `text` as one undoable edit.
    pub fn replace_range(&mut self, range: std::ops::Range<usize>, text: &str) {
        self.select(range);
        self.execute(Command::Insert(text.to_owned()));
    }

    /// Select `range` as the only selection (e.g. a search match).
    pub fn select(&mut self, range: std::ops::Range<usize>) {
        let len = self.doc.len_chars();
        self.goal = None;
        self.sels = Selections::single(Selection::new(range.start.min(len), range.end.min(len)));
    }

    /// Replace every match in one undoable step; returns how many.
    pub fn replace_all(&mut self, matcher: &sanain_core::search::Matcher, with: &str) -> usize {
        let (tx, n) = matcher.replace_all(self.doc.rope(), with);
        if n > 0 {
            let first = tx.caret_positions().first().copied().unwrap_or(0);
            if self.doc.apply(&tx).is_ok() {
                self.sels = Selections::single(Selection::cursor(first.min(self.doc.len_chars())));
            }
        }
        n
    }

    /// Replace the primary selection if it is exactly a match. Returns
    /// whether a replacement happened.
    pub fn replace_selected(&mut self, matcher: &sanain_core::search::Matcher, with: &str) -> bool {
        let range = self.sels.primary().range();
        if range.is_empty() {
            return false;
        }
        match matcher.replacement_for(self.doc.rope(), range.clone(), with) {
            Some(text) => {
                self.select(range);
                self.execute(Command::Insert(text));
                true
            }
            None => false,
        }
    }

    /// Text of the current selections for Copy.
    pub fn copy(&self) -> Option<String> {
        let t = sel::selected_text(self.doc.rope(), &self.sels);
        (!t.is_empty()).then_some(t)
    }

    fn commit(&mut self, tx: Transaction, next: Selections) {
        // Commands build transactions from current selections, which are
        // always within bounds; a failure here is a bug, and the document
        // is left unchanged by `apply` in that case.
        if self.doc.apply(&tx).is_ok() {
            self.sels = next;
            let rope = self.doc.rope();
            let mut folds: Vec<usize> = self
                .folds
                .iter()
                .map(|&p| rope.line_to_char(rope.char_to_line(tx.map_pos(p).min(rope.len_chars()))))
                .collect();
            folds.sort_unstable();
            folds.dedup();
            self.folds = folds;
            self.unfold_around_caret();
        } else {
            debug_assert!(false, "command produced an invalid transaction");
        }
    }

    /// Lines hidden by folds (cached per revision and fold set).
    pub fn fold_map(&mut self, tab: usize) -> &FoldMap {
        self.fold_tab = tab;
        let rev = self.doc.revision();
        let fresh = matches!(&self.fold_cache, Some((r, f, t, _)) if *r == rev && *f == self.folds && *t == tab);
        if !fresh {
            let rope = self.doc.rope();
            let ranges = self
                .folds
                .iter()
                .filter_map(|&p| {
                    folding::fold_range(rope, rope.char_to_line(p.min(rope.len_chars())), tab)
                })
                .collect();
            self.fold_cache = Some((rev, self.folds.clone(), tab, FoldMap::new(ranges)));
        }
        match &self.fold_cache {
            Some((_, _, _, m)) => m,
            None => unreachable!("cache was just filled"),
        }
    }

    pub fn is_folded(&self, line: usize) -> bool {
        let rope = self.doc.rope();
        line < rope.len_lines() && self.folds.contains(&rope.line_to_char(line))
    }

    /// Fold or unfold at `line` if it starts a fold. Returns whether
    /// anything changed.
    pub fn toggle_fold(&mut self, line: usize, tab: usize) -> bool {
        self.fold_tab = tab;
        let rope = self.doc.rope();
        if line >= rope.len_lines() {
            return false;
        }
        let at = rope.line_to_char(line);
        if let Some(i) = self.folds.iter().position(|&p| p == at) {
            self.folds.remove(i);
            return true;
        }
        if folding::fold_range(rope, line, tab).is_none() {
            return false;
        }
        self.folds.push(at);
        self.folds.sort_unstable();
        // Move the caret out of the hidden lines.
        let head_line = rope.char_to_line(self.sels.primary().head);
        if folding::fold_range(rope, line, tab).is_some_and(|r| r.contains(&head_line)) {
            let end = sel::line_content_end(rope, line);
            self.sels = Selections::single(Selection::cursor(end));
        }
        true
    }

    /// Fold the innermost region containing the caret.
    pub fn fold_at_caret(&mut self, tab: usize) -> bool {
        let rope = self.doc.rope();
        let line = rope.char_to_line(self.sels.primary().head);
        let start = (0..=line).rev().find(|&l| {
            l == line && folding::is_fold_start(rope, l, tab)
                || folding::fold_range(rope, l, tab).is_some_and(|r| r.contains(&line))
        });
        match start {
            Some(l) if !self.is_folded(l) => self.toggle_fold(l, tab),
            _ => false,
        }
    }

    /// Unfold the fold on the caret's line, if any.
    pub fn unfold_at_caret(&mut self) -> bool {
        let rope = self.doc.rope();
        let at = rope.line_to_char(rope.char_to_line(self.sels.primary().head));
        let before = self.folds.len();
        self.folds.retain(|&p| p != at);
        self.folds.len() != before
    }

    pub fn unfold_all(&mut self) -> bool {
        let any = !self.folds.is_empty();
        self.folds.clear();
        any
    }

    /// A caret moving into hidden lines opens the folds around it.
    fn unfold_around_caret(&mut self) {
        if self.folds.is_empty() {
            return;
        }
        let rope = self.doc.rope();
        let head = rope.char_to_line(self.sels.primary().head);
        let tab = self.fold_tab;
        self.folds.retain(|&p| {
            let l = rope.char_to_line(p.min(rope.len_chars()));
            !folding::fold_range(rope, l, tab).is_some_and(|r| r.contains(&head))
        });
    }

    pub fn save(&mut self) -> SaveOutcome {
        let Some(file) = &self.file else {
            return SaveOutcome::NeedsPath;
        };
        let (path, enc, fp) = (file.path.clone(), file.encoding, file.fingerprint.clone());
        self.write(&path, enc, SaveMode::Expect(&fp))
    }

    /// Save the file in a different encoding (explicit conversion). Fails
    /// without writing if a character cannot be represented.
    pub fn save_with_encoding(&mut self, encoding: TextEncoding) -> SaveOutcome {
        let Some(file) = &self.file else {
            self.untitled_encoding = encoding;
            return SaveOutcome::NeedsPath;
        };
        let (path, fp) = (file.path.clone(), file.fingerprint.clone());
        self.write(&path, encoding, SaveMode::Expect(&fp))
    }

    /// Save after the user confirmed overwriting an external change.
    pub fn save_overwrite(&mut self) -> SaveOutcome {
        let Some(file) = &self.file else {
            return SaveOutcome::NeedsPath;
        };
        let (path, enc) = (file.path.clone(), file.encoding);
        self.write(&path, enc, SaveMode::Overwrite)
    }

    /// Save to a path the user chose in a native dialog. The dialog already
    /// asked about replacing an existing file.
    pub fn save_as(&mut self, path: &Path) -> SaveOutcome {
        let enc = self.encoding();
        self.write(path, enc, SaveMode::Overwrite)
    }

    fn write(&mut self, path: &Path, encoding: TextEncoding, mode: SaveMode<'_>) -> SaveOutcome {
        match sanain_io::save(path, &self.doc.text(), encoding, mode) {
            Ok(fingerprint) => {
                self.doc.mark_saved();
                self.file = Some(FileBinding {
                    path: path.to_owned(),
                    encoding,
                    fingerprint,
                });
                SaveOutcome::Saved
            }
            Err(SaveError::ExternalChange { .. }) => SaveOutcome::Conflict,
            Err(SaveError::ReadOnly { .. }) => SaveOutcome::ReadOnly,
            Err(e) => SaveOutcome::Failed(e),
        }
    }
}

fn snap(rope: &ropey::Rope, at: usize) -> usize {
    let at = at.min(rope.len_chars());
    if sanain_core::grapheme::is_boundary(rope, at) {
        at
    } else {
        sanain_core::grapheme::prev_boundary(rope, at)
    }
}

fn platform_newline() -> &'static str {
    if cfg!(windows) { "\r\n" } else { "\n" }
}

/// New lines follow the file's first line break; files without one use the
/// platform convention.
fn preferred_newline(text: &str) -> &'static str {
    match text.find(['\n', '\r']) {
        Some(i) if text[i..].starts_with("\r\n") => "\r\n",
        Some(i) if text[i..].starts_with('\r') => "\r",
        Some(_) => "\n",
        None => platform_newline(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn replacing_whole_text_is_one_undo_step() {
        let mut ed = Editor::new();
        ed.execute(Command::Insert("version one two three\n".into()));
        let len = ed.document().len_chars();
        ed.replace_range(0..len, "version one two\n");
        ed.select(0..0);
        assert_eq!(ed.document().text(), "version one two\n");
        ed.execute(Command::Undo);
        assert_eq!(ed.document().text(), "version one two three\n");
    }

    #[test]
    fn typing_newline_follows_file_convention() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("crlf.txt");
        fs::write(&path, "a\r\nb").unwrap();
        let mut ed = Editor::open(&path).unwrap();
        ed.execute(Command::LineEnd { extend: false });
        ed.execute(Command::NewLine);
        ed.execute(Command::Insert("x".into()));
        assert_eq!(ed.document().text(), "a\r\nx\r\nb");
        assert!(ed.save().is_saved());
        assert_eq!(fs::read(&path).unwrap(), b"a\r\nx\r\nb");
        assert!(!ed.document().is_dirty());
    }

    #[test]
    fn untitled_needs_path_then_saves() {
        let dir = tempfile::tempdir().unwrap();
        let mut ed = Editor::new();
        ed.execute(Command::Insert("hello".into()));
        assert!(matches!(ed.save(), SaveOutcome::NeedsPath));
        let path = dir.path().join("new.txt");
        assert!(ed.save_as(&path).is_saved());
        assert_eq!(ed.file_name().as_deref(), Some("new.txt"));
        ed.execute(Command::Insert("!".into()));
        assert!(ed.save().is_saved());
        assert_eq!(fs::read_to_string(&path).unwrap(), "hello!");
    }

    #[test]
    fn conflict_then_explicit_overwrite() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("c.txt");
        fs::write(&path, "v1").unwrap();
        let mut ed = Editor::open(&path).unwrap();
        ed.execute(Command::Insert("mine ".into()));
        fs::write(&path, "theirs").unwrap();
        assert!(matches!(ed.save(), SaveOutcome::Conflict));
        assert!(ed.document().is_dirty(), "still dirty after refused save");
        assert_eq!(fs::read_to_string(&path).unwrap(), "theirs");
        assert!(ed.save_overwrite().is_saved());
        assert_eq!(fs::read_to_string(&path).unwrap(), "mine v1");
    }

    #[test]
    fn cut_copy_undo_restores_caret() {
        let mut ed = Editor::new();
        ed.execute(Command::Insert("hello world".into()));
        ed.execute(Command::PlaceCaret {
            at: 5,
            extend: false,
        });
        ed.execute(Command::PlaceCaret {
            at: 11,
            extend: true,
        });
        assert_eq!(ed.copy().as_deref(), Some(" world"));
        assert_eq!(ed.execute(Command::Cut).as_deref(), Some(" world"));
        assert_eq!(ed.document().text(), "hello");
        ed.execute(Command::Undo);
        assert_eq!(ed.document().text(), "hello world");
        assert_eq!(ed.selections().primary(), Selection::cursor(11));
    }

    #[test]
    fn vertical_moves_keep_goal_column_across_short_lines() {
        let mut ed = Editor::new();
        ed.execute(Command::Insert("abcdef\nx\nabcdef".into()));
        ed.execute(Command::PlaceCaret {
            at: 5,
            extend: false,
        });
        ed.execute(Command::Down { extend: false });
        assert_eq!(
            ed.selections().primary(),
            Selection::cursor(8),
            "clamped on 'x'"
        );
        ed.execute(Command::Down { extend: false });
        assert_eq!(
            ed.selections().primary(),
            Selection::cursor(14),
            "back to column 5"
        );
        // Any other command resets the goal.
        ed.execute(Command::Left { extend: false });
        ed.execute(Command::Up { extend: false });
        assert_eq!(ed.selections().primary(), Selection::cursor(8));
    }

    #[test]
    fn new_breaks_follow_the_document_not_the_platform() {
        let mut ed = Editor::new();
        ed.newline = "\r\n"; // as on Windows
        ed.execute(Command::Insert("a\nb".into()));
        ed.execute(Command::NewLine);
        ed.execute(Command::DuplicateLine);
        assert_eq!(ed.document().text(), "a\nb\n\n");
        let mut ed = Editor::new();
        ed.newline = "\n";
        ed.execute(Command::Insert("x\r\ny".into()));
        ed.execute(Command::NewLine);
        assert_eq!(ed.document().text(), "x\r\ny\r\n");
    }

    #[test]
    fn line_commands_round_trip() {
        let mut ed = Editor::new();
        ed.execute(Command::Insert("one\ntwo".into()));
        ed.execute(Command::PlaceCaret {
            at: 1,
            extend: false,
        });
        ed.execute(Command::MoveLineDown);
        assert_eq!(ed.document().text(), "two\none");
        ed.execute(Command::DuplicateLine);
        assert_eq!(ed.document().text(), "two\none\none");
        ed.execute(Command::DeleteLine);
        assert_eq!(ed.document().text(), "two\none");
        ed.execute(Command::Undo);
        ed.execute(Command::Undo);
        ed.execute(Command::Undo);
        assert_eq!(ed.document().text(), "one\ntwo");
    }

    #[test]
    fn replace_selected_and_all() {
        use sanain_core::search::Query;
        let mut ed = Editor::new();
        ed.execute(Command::Insert("a1 a2 a3".into()));
        let m = Query {
            pattern: r"a(\d)".into(),
            regex: true,
            ..Default::default()
        }
        .compile()
        .unwrap()
        .unwrap();
        ed.select(0..1);
        assert!(
            !ed.replace_selected(&m, "b$1"),
            "partial match is not replaced"
        );
        ed.select(3..5);
        assert!(ed.replace_selected(&m, "b$1"));
        assert_eq!(ed.document().text(), "a1 b2 a3");
        assert_eq!(ed.replace_all(&m, "c$1"), 2);
        assert_eq!(ed.document().text(), "c1 b2 c3");
        ed.execute(Command::Undo);
        assert_eq!(ed.document().text(), "a1 b2 a3", "replace all is one step");
    }

    #[test]
    fn transform_selection_or_whole_document_in_one_undo() {
        let mut ed = Editor::new();
        ed.execute(Command::Insert("b\na\nc".into()));
        assert_eq!(
            ed.transform(|t| Ok::<_, ()>(sanain_tools::sort_lines(t, false))),
            Ok(true)
        );
        assert_eq!(ed.document().text(), "a\nb\nc");
        ed.select(0..3);
        assert_eq!(ed.transform(|t| Ok::<_, ()>(t.to_uppercase())), Ok(true));
        assert_eq!(ed.document().text(), "A\nB\nc");
        assert_eq!(ed.selections().primary().range(), 0..3);
        assert_eq!(
            ed.transform(|t| Ok::<_, ()>(t.to_owned())),
            Ok(false),
            "no-op"
        );
        assert_eq!(ed.transform(|_| Err::<String, _>("bad")), Err("bad"));
        ed.execute(Command::Undo);
        assert_eq!(ed.document().text(), "a\nb\nc");
    }

    #[test]
    fn reopen_as_legacy_and_convert_to_utf8() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("w.txt");
        fs::write(&path, b"na\xEFve\n").unwrap();
        assert!(Editor::open(&path).is_err());
        let enc = TextEncoding::Legacy(encoding_rs::WINDOWS_1252);
        let mut ed = Editor::open_as(&path, enc).unwrap();
        assert_eq!(ed.document().text(), "naïve\n");
        assert_eq!(ed.encoding(), enc);
        assert!(ed.save_with_encoding(TextEncoding::Utf8).is_saved());
        assert_eq!(fs::read(&path).unwrap(), "naïve\n".as_bytes());
        assert_eq!(ed.encoding(), TextEncoding::Utf8);
        ed.execute(Command::Insert("漢".into()));
        assert!(matches!(ed.save_with_encoding(enc), SaveOutcome::Failed(_)));
        assert_eq!(fs::read(&path).unwrap(), "naïve\n".as_bytes(), "unchanged");
    }

    #[test]
    fn folds_follow_edits_and_open_for_the_caret() {
        let mut ed = Editor::new();
        ed.execute(Command::Insert("a {\n  b\n  c\n}\nd\n".into()));
        ed.execute(Command::PlaceCaret {
            at: 0,
            extend: false,
        });
        assert!(ed.toggle_fold(0, 4));
        assert_eq!(ed.fold_map(4).hidden(), std::slice::from_ref(&(1..3)));
        // Typing above keeps the fold on its line.
        ed.execute(Command::Insert("x\n".into()));
        assert!(ed.is_folded(1));
        assert_eq!(ed.fold_map(4).hidden(), std::slice::from_ref(&(2..4)));
        // Moving into the hidden region opens it.
        ed.execute(Command::PlaceCaret {
            at: 7,
            extend: false,
        });
        assert!(ed.fold_map(4).is_empty());
        assert!(ed.fold_at_caret(4));
        assert!(!ed.fold_map(4).is_empty());
        assert!(ed.unfold_all());
    }

    #[test]
    fn place_caret_snaps_out_of_grapheme() {
        let mut ed = Editor::new();
        ed.execute(Command::Insert("e\u{301}x".into()));
        ed.execute(Command::PlaceCaret {
            at: 1,
            extend: false,
        });
        assert_eq!(ed.selections().primary(), Selection::cursor(0));
    }

    #[test]
    fn preferred_newline_detection() {
        assert_eq!(preferred_newline("a\nb\r\n"), "\n");
        assert_eq!(preferred_newline("a\r\nb\n"), "\r\n");
        assert_eq!(preferred_newline("a\rb"), "\r");
        assert_eq!(preferred_newline("a"), platform_newline());
    }

    impl SaveOutcome {
        fn is_saved(&self) -> bool {
            matches!(self, SaveOutcome::Saved)
        }
    }
}
