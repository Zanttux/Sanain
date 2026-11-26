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
    /// Add a caret at `at`, or remove the selection there if there are
    /// several.
    AddCaret {
        at: usize,
    },
    /// Add a caret on the line above the topmost (or below the
    /// bottommost) selection, at the same column.
    AddCaretVertical {
        down: bool,
    },
    /// Replace all selections: (anchor, head) pairs; the last is primary.
    SetSelections(Vec<(usize, usize)>),
    /// Keep only the primary selection.
    SingleSelection,
    /// Indent the selected lines by the given text.
    IndentLines(String),
    /// Remove one indentation level (tab or up to N spaces).
    OutdentLines(usize),
    /// Comment out or uncomment the selected lines with this prefix.
    ToggleComment(String),
    /// With an empty selection, select the word at the caret; otherwise
    /// add the next occurrence of the selected text (wrapping around).
    SelectNextOccurrence,
    /// Move the newest selection to the next occurrence of its text.
    SkipOccurrence,
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
    /// Commands recorded for a macro, while recording.
    macro_rec: Option<Vec<Command>>,
    line_ending_cache: std::cell::Cell<Option<(sanain_core::Revision, LineEnding)>>,
    /// Folded regions, as the char index where each fold's start line
    /// begins. Mapped through edits so folds stay on their lines.
    folds: Vec<usize>,
    /// Tab width the folds were computed with (from settings).
    fold_tab: usize,
    /// Cached hidden-line map: (revision, folds, tab width, map).
    fold_cache: Option<(sanain_core::Revision, Vec<usize>, usize, FoldMap)>,
    /// Syntax fold ranges for a revision (see [`Self::set_syntax_folds`]);
    /// indentation folding applies when absent or stale.
    syntax_folds: Option<(sanain_core::Revision, Vec<std::ops::Range<usize>>)>,
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
            macro_rec: None,
            line_ending_cache: std::cell::Cell::new(None),
            folds: Vec::new(),
            fold_tab: 4,
            fold_cache: None,
            syntax_folds: None,
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
            macro_rec: None,
            line_ending_cache: std::cell::Cell::new(None),
            folds: Vec::new(),
            fold_tab: 4,
            fold_cache: None,
            syntax_folds: None,
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

    /// The document's line-ending style, cached per revision (the status
    /// bar asks every frame; scanning is linear in the text size).
    pub fn line_ending(&self) -> LineEnding {
        let rev = self.doc.revision();
        let detected = match self.line_ending_cache.get() {
            Some((r, le)) if r == rev => le,
            _ => {
                let le = LineEnding::detect_chunks(self.doc.rope().chunks());
                self.line_ending_cache.set(Some((rev, le)));
                le
            }
        };
        match detected {
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
        // With nothing selected, Skip first selects the word like Select Next.
        let cmd = if cmd == Command::SkipOccurrence && self.sels.primary().range().is_empty() {
            Command::SelectNextOccurrence
        } else {
            cmd
        };
        if let Some(rec) = &mut self.macro_rec
            && cmd.is_recordable()
        {
            rec.push(cmd.clone());
        }
        let goal = self.goal.take();
        let rope = self.doc.rope();
        let (tx, next): (Option<Transaction>, Selections) = match cmd {
            Command::Insert(text) => {
                let (tx, s) = sel::insert(&self.sels, &text);
                (Some(tx), s)
            }
            Command::NewLine => {
                let (tx, s) = sel::newline_indented(rope, &self.sels, self.newline());
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
            Command::AddCaret { at } => {
                let at = snap(rope, at);
                let mut items: Vec<Selection> = self.sels.iter().copied().collect();
                let hit = items
                    .iter()
                    .position(|s| s.range().contains(&at) || s.head == at);
                match hit {
                    Some(i) if items.len() > 1 => {
                        items.remove(i);
                        let p = items.len() - 1;
                        (None, Selections::new(items, p, rope.len_chars()))
                    }
                    Some(_) => return None,
                    None => {
                        items.push(Selection::cursor(at));
                        let p = items.len() - 1;
                        (None, Selections::new(items, p, rope.len_chars()))
                    }
                }
            }
            Command::AddCaretVertical { down } => {
                let items: Vec<Selection> = self.sels.iter().copied().collect();
                let line_of = |s: &Selection| rope.char_to_line(s.head);
                let edge = if down {
                    items.iter().max_by_key(|s| line_of(s))
                } else {
                    items.iter().min_by_key(|s| line_of(s))
                };
                let edge = edge.copied()?;
                let line = line_of(&edge);
                // Repeated presses keep the first column (like Up/Down).
                let col = match goal.as_deref() {
                    Some(&[c]) => c,
                    _ => edge.head - rope.line_to_char(line),
                };
                self.goal = Some(vec![col]);
                let target = if down {
                    (line + 1 < rope.len_lines()).then_some(line + 1)
                } else {
                    line.checked_sub(1)
                };
                let t = target?;
                let start = rope.line_to_char(t);
                let at = snap(rope, (start + col).min(sel::line_content_end(rope, t)));
                let mut items = items;
                items.push(Selection::cursor(at));
                let p = items.len() - 1;
                (None, Selections::new(items, p, rope.len_chars()))
            }
            Command::SetSelections(pairs) => {
                let len = rope.len_chars();
                let items: Vec<Selection> = pairs
                    .into_iter()
                    .map(|(a, h)| Selection::new(snap(rope, a.min(len)), snap(rope, h.min(len))))
                    .collect();
                let p = items.len().saturating_sub(1);
                (None, Selections::new(items, p, len))
            }
            Command::SingleSelection => (None, Selections::single(self.sels.primary())),
            Command::IndentLines(indent) => {
                let (tx, s) = lines::indent_lines(rope, &self.sels, &indent);
                (Some(tx), s)
            }
            Command::OutdentLines(tab) => {
                let (tx, s) = lines::outdent_lines(rope, &self.sels, tab);
                (Some(tx), s)
            }
            Command::ToggleComment(prefix) => {
                let (tx, s) = lines::toggle_comment(rope, &self.sels, &prefix);
                (Some(tx), s)
            }
            Command::SkipOccurrence => {
                let primary = self.sels.primary();
                let len = rope.len_chars();
                let needle: Vec<char> = rope.slice(primary.range()).chars().collect();
                let n = needle.len();
                let end = primary.range().end;
                let others = |start: usize| {
                    self.sels
                        .iter()
                        .any(|s| *s != primary && s.range().start == start)
                };
                let matches_at = |start: usize| {
                    start + n <= len
                        && rope
                            .slice(start..start + n)
                            .chars()
                            .eq(needle.iter().copied())
                };
                let start = (end..len)
                    .chain(0..primary.range().start)
                    .find(|&s| matches_at(s) && !others(s))?;
                let mut items: Vec<Selection> = self
                    .sels
                    .iter()
                    .copied()
                    .filter(|s| *s != primary)
                    .collect();
                items.push(Selection::new(start, start + n));
                let p = items.len() - 1;
                (None, Selections::new(items, p, len))
            }
            Command::SelectNextOccurrence => {
                let primary = self.sels.primary();
                let len = rope.len_chars();
                let is_word = |c: char| c.is_alphanumeric() || c == '_';
                if primary.range().is_empty() {
                    let at = primary.head;
                    let mut a = at;
                    while a > 0 && is_word(rope.char(a - 1)) {
                        a -= 1;
                    }
                    let mut b = at;
                    while b < len && is_word(rope.char(b)) {
                        b += 1;
                    }
                    if a == b {
                        return None;
                    }
                    let mut items: Vec<Selection> = self.sels.iter().copied().collect();
                    let p = items.iter().position(|s| *s == primary).unwrap_or(0);
                    items[p] = Selection::new(a, b);
                    (None, Selections::new(items, p, len))
                } else {
                    let needle: Vec<char> = rope.slice(primary.range()).chars().collect();
                    let n = needle.len();
                    let last_end = self.sels.iter().map(|s| s.range().end).max().unwrap_or(0);
                    let taken = |start: usize| self.sels.iter().any(|s| s.range().start == start);
                    let matches_at = |start: usize| {
                        start + n <= len
                            && rope.char(start) == needle[0]
                            && rope
                                .slice(start..start + n)
                                .chars()
                                .eq(needle.iter().copied())
                    };
                    // Search forward from the last selection, then wrap.
                    let found = (last_end..len)
                        .chain(0..last_end)
                        .find(|&s| matches_at(s) && !taken(s));
                    let start = found?;
                    let mut items: Vec<Selection> = self.sels.iter().copied().collect();
                    items.push(Selection::new(start, start + n));
                    let p = items.len() - 1;
                    (None, Selections::new(items, p, len))
                }
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

    /// Put the caret at 0-based `line`, `col` (clamped to the document
    /// and the line's length).
    pub fn go_to(&mut self, line: usize, col: usize) {
        let rope = self.doc.rope();
        let line = line.min(rope.len_lines().saturating_sub(1));
        let start = rope.line_to_char(line);
        let at = (start + col).min(sel::line_content_end(rope, line));
        self.execute(Command::PlaceCaret { at, extend: false });
    }

    /// Start recording editing commands for a macro.
    pub fn start_recording(&mut self) {
        self.macro_rec = Some(Vec::new());
    }

    pub fn is_recording(&self) -> bool {
        self.macro_rec.is_some()
    }

    /// Stop recording and return the recorded commands.
    pub fn stop_recording(&mut self) -> Vec<Command> {
        self.macro_rec.take().unwrap_or_default()
    }

    /// Run `commands` at the current selections; the edits undo as one
    /// step.
    pub fn play(&mut self, commands: &[Command]) {
        self.play_times(commands, 1);
    }

    /// Play `commands` `times` times as one undo step.
    pub fn play_times(&mut self, commands: &[Command], times: usize) {
        let mark = self.doc.undo_mark();
        let rec = self.macro_rec.take();
        for _ in 0..times {
            for c in commands {
                self.execute(c.clone());
            }
        }
        self.macro_rec = rec;
        self.doc.merge_undo_since(mark);
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
        self.apply_replacement(&tx);
        n
    }

    /// Apply a prepared replace-all transaction as one undoable step.
    pub fn apply_replacement(&mut self, tx: &Transaction) {
        if tx.is_empty() {
            return;
        }
        let first = tx.caret_positions().first().copied().unwrap_or(0);
        if self.doc.apply(tx).is_ok() {
            self.sels = Selections::single(Selection::cursor(first.min(self.doc.len_chars())));
        }
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
        // Cheap: ropes share structure.
        let before = self.syntax_folds.is_some().then(|| self.doc.rope().clone());
        if self.doc.apply(&tx).is_ok() {
            self.sels = next;
            if let Some(before) = before {
                self.map_syntax_folds(&before, &tx);
            }
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

    /// Use syntax-tree fold ranges computed for `revision` (from
    /// [`sanain_language::Highlighter::folds`]: each range starts a fold
    /// on `start` and hides `start + 1..end`). Results for another
    /// revision are ignored; `None` returns to indentation folding.
    pub fn set_syntax_folds(
        &mut self,
        revision: sanain_core::Revision,
        folds: Option<Vec<std::ops::Range<usize>>>,
    ) {
        if revision != self.doc.revision() {
            return;
        }
        match folds {
            Some(f) => self.syntax_folds = Some((revision, f)),
            None => self.clear_syntax_folds(),
        }
        self.fold_cache = None;
    }

    /// Fold by indentation (no syntax tree for this buffer).
    pub fn clear_syntax_folds(&mut self) {
        if self.syntax_folds.take().is_some() {
            self.fold_cache = None;
        }
    }

    /// Carry syntax folds through an edit until the next parse arrives,
    /// so folded regions do not jump while typing.
    fn map_syntax_folds(&mut self, before: &ropey::Rope, tx: &Transaction) {
        let Some((rev, folds)) = self.syntax_folds.take() else {
            return;
        };
        let rope = self.doc.rope();
        // Mapping costs folds × changes; large edits wait for the parse.
        if rev.0 + 1 != self.doc.revision().0 || folds.len() * tx.changes().len() > 1 << 20 {
            return;
        }
        let map = |line: usize| {
            let at = tx.map_pos(before.line_to_char(line.min(before.len_lines())));
            rope.char_to_line(at.min(rope.len_chars()))
        };
        let mut mapped: Vec<std::ops::Range<usize>> = folds
            .into_iter()
            .map(|r| {
                let end = if r.end >= before.len_lines() {
                    rope.len_lines()
                } else {
                    map(r.end)
                };
                map(r.start)..end
            })
            .filter(|r| r.end > r.start + 1)
            .collect();
        mapped.sort_by(|a, b| a.start.cmp(&b.start).then(b.end.cmp(&a.end)));
        mapped.dedup_by_key(|r| r.start);
        self.syntax_folds = Some((self.doc.revision(), mapped));
    }

    /// Lines hidden when folding at `line`: from the syntax tree when
    /// available, else by indentation.
    fn fold_range(&self, line: usize, tab: usize) -> Option<std::ops::Range<usize>> {
        match &self.syntax_folds {
            Some((rev, folds)) if *rev == self.doc.revision() => {
                let i = folds.binary_search_by_key(&line, |r| r.start).ok()?;
                Some(line + 1..folds[i].end)
            }
            _ => folding::fold_range(self.doc.rope(), line, tab),
        }
    }

    /// Whether `line` starts a fold (for gutter markers).
    pub fn is_fold_start(&self, line: usize, tab: usize) -> bool {
        match &self.syntax_folds {
            Some((rev, folds)) if *rev == self.doc.revision() => {
                folds.binary_search_by_key(&line, |r| r.start).is_ok()
            }
            _ => folding::is_fold_start(self.doc.rope(), line, tab),
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
                .filter_map(|&p| self.fold_range(rope.char_to_line(p.min(rope.len_chars())), tab))
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
        let Some(hidden) = self.fold_range(line, tab) else {
            return false;
        };
        self.folds.push(at);
        self.folds.sort_unstable();
        // Move the caret out of the hidden lines.
        let head_line = rope.char_to_line(self.sels.primary().head);
        if hidden.contains(&head_line) {
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
            l == line && self.is_fold_start(l, tab)
                || self.fold_range(l, tab).is_some_and(|r| r.contains(&line))
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
        let mut folds = std::mem::take(&mut self.folds);
        folds.retain(|&p| {
            let l = rope.char_to_line(p.min(rope.len_chars()));
            !self.fold_range(l, tab).is_some_and(|r| r.contains(&head))
        });
        self.folds = folds;
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

/// Parse a Go to Line target: `line` or `line:column`, both 1-based.
/// Returns 0-based (line, column).
pub fn parse_line_target(s: &str) -> Option<(usize, usize)> {
    let s = s.trim();
    let (l, c) = match s.split_once([':', ',']) {
        Some((l, c)) => (l.trim(), Some(c.trim())),
        None => (s, None),
    };
    let line: usize = l.parse().ok()?;
    let col: usize = match c {
        Some(c) => c.parse().ok()?,
        None => 1,
    };
    Some((line.max(1) - 1, col.max(1) - 1))
}

impl Command {
    /// Whether a macro records this command. Pointer placement, undo and
    /// redo are left out: absolute positions and history steps do not
    /// replay meaningfully.
    pub fn is_recordable(&self) -> bool {
        !matches!(
            self,
            Command::PlaceCaret { .. }
                | Command::AddCaret { .. }
                | Command::SetSelections(_)
                | Command::Undo
                | Command::Redo
        )
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
    fn macro_records_and_replays_as_one_undo() {
        let mut ed = Editor::new();
        ed.execute(Command::Insert("alpha\nbeta\ngamma\n".into()));
        ed.execute(Command::PlaceCaret {
            at: 0,
            extend: false,
        });
        ed.start_recording();
        ed.execute(Command::Insert("- ".into()));
        ed.execute(Command::LineEnd { extend: false });
        ed.execute(Command::Insert(";".into()));
        ed.execute(Command::Down { extend: false });
        ed.execute(Command::LineStart { extend: false });
        ed.execute(Command::PlaceCaret {
            at: 0,
            extend: false,
        }); // not recorded
        let m = ed.stop_recording();
        assert_eq!(m.len(), 5);
        assert!(!ed.is_recording());
        ed.execute(Command::Undo);
        ed.execute(Command::Undo);
        ed.execute(Command::PlaceCaret {
            at: 6,
            extend: false,
        });
        ed.play(&m);
        ed.play(&m);
        assert_eq!(ed.document().text(), "alpha\n- beta;\n- gamma;\n");
        ed.execute(Command::Undo);
        assert_eq!(
            ed.document().text(),
            "alpha\n- beta;\ngamma\n",
            "one run, one undo"
        );
    }

    #[test]
    fn play_times_is_one_undo() {
        let mut ed = Editor::new();
        ed.execute(Command::Insert("a\nb\nc\n".into()));
        ed.execute(Command::PlaceCaret {
            at: 0,
            extend: false,
        });
        let cmds = vec![
            Command::Insert("- ".into()),
            Command::Down { extend: false },
            Command::LineStart { extend: false },
        ];
        ed.play_times(&cmds, 3);
        assert_eq!(ed.document().text(), "- a\n- b\n- c\n");
        ed.execute(Command::Undo);
        assert_eq!(ed.document().text(), "a\nb\nc\n", "all runs undo together");
    }

    #[test]
    fn skip_occurrence() {
        let mut ed = Editor::new();
        ed.execute(Command::Insert("ab x ab y ab z ab".into()));
        ed.execute(Command::PlaceCaret {
            at: 0,
            extend: false,
        });
        ed.execute(Command::SkipOccurrence);
        assert_eq!(
            ed.copy().as_deref(),
            Some("ab"),
            "empty selection: select word"
        );
        ed.execute(Command::SelectNextOccurrence); // ab at 5
        ed.execute(Command::SkipOccurrence); // 5 -> 10
        ed.execute(Command::SelectNextOccurrence); // + 15
        let starts: Vec<usize> = ed.selections().iter().map(|s| s.range().start).collect();
        assert_eq!(
            starts,
            vec![0, 10, 15],
            "the skipped occurrence is not selected"
        );
        ed.execute(Command::SkipOccurrence); // 15 wraps past taken 0 and 10 -> 5
        let starts: Vec<usize> = ed.selections().iter().map(|s| s.range().start).collect();
        assert_eq!(starts, vec![0, 5, 10]);
        ed.execute(Command::Insert("Q".into()));
        assert_eq!(ed.document().text(), "Q x Q y Q z ab");
    }

    #[test]
    fn select_next_occurrence() {
        let mut ed = Editor::new();
        ed.execute(Command::Insert("foo bar foo_x foo\nfoo".into()));
        ed.execute(Command::PlaceCaret {
            at: 1,
            extend: false,
        });
        ed.execute(Command::SelectNextOccurrence);
        assert_eq!(ed.copy().as_deref(), Some("foo"), "word at caret");
        ed.execute(Command::SelectNextOccurrence);
        ed.execute(Command::SelectNextOccurrence);
        ed.execute(Command::SelectNextOccurrence);
        let starts: Vec<usize> = ed.selections().iter().map(|s| s.range().start).collect();
        assert_eq!(
            starts,
            vec![0, 8, 14, 18],
            "substring matches too, in order"
        );
        ed.execute(Command::SelectNextOccurrence);
        assert_eq!(ed.selections().len(), 4, "all taken: no-op");
        ed.execute(Command::Insert("baz".into()));
        assert_eq!(ed.document().text(), "baz bar baz_x baz\nbaz");
        // Wraps around to occurrences before the selection.
        ed.execute(Command::PlaceCaret {
            at: 19,
            extend: false,
        });
        ed.execute(Command::SelectNextOccurrence);
        ed.execute(Command::SelectNextOccurrence);
        assert_eq!(
            ed.selections().iter().next().map(|s| s.range().start),
            Some(0)
        );
    }

    #[test]
    fn go_to_line_targets() {
        assert_eq!(parse_line_target("12"), Some((11, 0)));
        assert_eq!(parse_line_target(" 3:7 "), Some((2, 6)));
        assert_eq!(parse_line_target("3,7"), Some((2, 6)));
        assert_eq!(parse_line_target("0"), Some((0, 0)));
        for bad in ["", "x", "3:", "-1", "1:y"] {
            assert_eq!(parse_line_target(bad), None, "{bad}");
        }
        let mut ed = Editor::new();
        ed.execute(Command::Insert("one\ntwo\nthree".into()));
        ed.go_to(1, 99);
        assert_eq!(ed.selections().primary().head, 7, "clamped to line end");
        ed.go_to(99, 2);
        assert_eq!(ed.selections().primary().head, 10, "last line");
    }

    #[test]
    fn creating_multiple_cursors() {
        let mut ed = Editor::new();
        ed.execute(Command::Insert("abcd\nab\nabcdef\n".into()));
        ed.execute(Command::PlaceCaret {
            at: 3,
            extend: false,
        });
        ed.execute(Command::AddCaretVertical { down: true });
        ed.execute(Command::AddCaretVertical { down: true });
        let heads: Vec<usize> = ed.selections().iter().map(|s| s.head).collect();
        assert_eq!(heads, vec![3, 7, 11], "column clamped on the short line");
        ed.execute(Command::Insert("X".into()));
        assert_eq!(ed.document().text(), "abcXd\nabX\nabcXdef\n");
        ed.execute(Command::Undo);
        // Alt+click toggles: add, then remove again.
        ed.execute(Command::PlaceCaret {
            at: 0,
            extend: false,
        });
        ed.execute(Command::AddCaret { at: 5 });
        assert_eq!(ed.selections().len(), 2);
        ed.execute(Command::AddCaret { at: 5 });
        assert_eq!(ed.selections().len(), 1);
        ed.execute(Command::AddCaret { at: 0 });
        assert_eq!(ed.selections().len(), 1, "never removes the last caret");
        // Block selection, then back to one.
        ed.execute(Command::SetSelections(vec![(1, 3), (6, 7), (9, 11)]));
        assert_eq!(ed.copy().as_deref(), Some("bc\nb\nbc"));
        assert_eq!(ed.selections().primary().head, 11);
        ed.execute(Command::SingleSelection);
        assert_eq!(ed.selections().len(), 1);
        assert_eq!(ed.selections().primary().head, 11);
    }

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
    fn syntax_folds_replace_indentation_folds() {
        let src = "int main(void)\n{\nif (x) {\nreturn 1;\n}\n  return 0;\n    }\n";
        let mut ed = Editor::new();
        ed.execute(Command::Insert(src.into()));
        ed.execute(Command::PlaceCaret {
            at: 0,
            extend: false,
        });
        // Indentation folding finds nothing at the `if`.
        assert!(!ed.is_fold_start(2, 4));
        assert!(!ed.toggle_fold(2, 4));
        let folds = sanain_language::Highlighter::new()
            .folds(sanain_language::Lang::C, src.as_bytes(), None)
            .unwrap();
        // A result for an older revision is ignored.
        let rev = ed.document().revision();
        ed.set_syntax_folds(sanain_core::Revision(rev.0 - 1), Some(folds.clone()));
        assert!(!ed.is_fold_start(2, 4));
        ed.set_syntax_folds(rev, Some(folds));
        assert!(ed.is_fold_start(2, 4) && ed.is_fold_start(0, 4));
        assert!(!ed.is_fold_start(3, 4));
        // Folding the `if` hides its body only; `}` stays visible.
        assert!(ed.toggle_fold(2, 4));
        assert_eq!(ed.fold_map(4).hidden(), std::slice::from_ref(&(3..4)));
        assert!(ed.toggle_fold(0, 4));
        assert_eq!(ed.fold_map(4).hidden(), std::slice::from_ref(&(1..6)));
        assert!(ed.toggle_fold(0, 4), "unfold");
        // An edit above carries the folds down a line until the next parse.
        ed.execute(Command::Insert("// x\n".into()));
        assert!(ed.is_fold_start(3, 4) && !ed.is_fold_start(0, 4));
        assert!(ed.is_folded(3));
        assert_eq!(ed.fold_map(4).hidden(), std::slice::from_ref(&(4..5)));
        assert!(ed.unfold_all());
        // Back to indentation folding.
        ed.clear_syntax_folds();
        assert!(!ed.is_fold_start(3, 4));
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

    #[test]
    fn csv_cell_edit_is_one_undo_step_and_saves_only_that_field() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("t.csv");
        let original = "id,name,note\r\n1,\"Äiti\",🙂 ok\r\n2,Ville,x";
        fs::write(&path, original).unwrap();
        let mut ed = Editor::open(&path).unwrap();
        let table = sanain_tools::csv::parse(&ed.document().text(), ',');
        let edit = table.edit_cell(2, 1, "Kalle, \"K\"").unwrap();
        ed.replace_range(edit.range, &edit.text);
        let edited = "id,name,note\r\n1,\"Äiti\",🙂 ok\r\n2,\"Kalle, \"\"K\"\"\",x";
        assert_eq!(ed.document().text(), edited);
        assert!(ed.document().is_dirty());
        ed.execute(Command::Undo);
        assert_eq!(ed.document().text(), original, "one undo restores all");
        assert!(!ed.document().is_dirty());
        ed.execute(Command::Redo);
        assert!(matches!(ed.save(), SaveOutcome::Saved));
        assert_eq!(fs::read(&path).unwrap(), edited.as_bytes());
    }
}
