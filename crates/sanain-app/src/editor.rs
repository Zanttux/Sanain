//! One open document plus its view state and file binding. No GUI types.

use std::path::{Path, PathBuf};

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
        let loaded = sanain_io::load(path)?;
        Ok(Self {
            newline: preferred_newline(&loaded.text),
            goal: None,
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
                }
                return None;
            }
        };
        match tx {
            Some(tx) => self.commit(tx, next),
            None => self.sels = next,
        }
        None
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
        } else {
            debug_assert!(false, "command produced an invalid transaction");
        }
    }

    pub fn save(&mut self) -> SaveOutcome {
        let Some(file) = &self.file else {
            return SaveOutcome::NeedsPath;
        };
        let (path, enc, fp) = (file.path.clone(), file.encoding, file.fingerprint.clone());
        self.write(&path, enc, SaveMode::Expect(&fp))
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
