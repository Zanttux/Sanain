//! One open document plus its view state and file binding. No GUI types.

use std::path::{Path, PathBuf};

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
    newline: &'static str,
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
            newline: platform_newline(),
        }
    }

    pub fn open(path: &Path) -> Result<Self, LoadError> {
        let loaded = sanain_io::load(path)?;
        Ok(Self {
            newline: preferred_newline(&loaded.text),
            doc: Document::from_saved_text(&loaded.text),
            sels: Selections::single(Selection::cursor(0)),
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
            .map_or(TextEncoding::Utf8, |f| f.encoding)
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

    /// Run a command. Returns text for the clipboard (Cut) if any.
    pub fn execute(&mut self, cmd: Command) -> Option<String> {
        let rope = self.doc.rope();
        let (tx, next): (Option<Transaction>, Selections) = match cmd {
            Command::Insert(text) => {
                let (tx, s) = sel::insert(&self.sels, &text);
                (Some(tx), s)
            }
            Command::NewLine => {
                let (tx, s) = sel::insert(&self.sels, self.newline);
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
            Command::Left { extend } => {
                (None, sel::move_horizontal(rope, &self.sels, false, extend))
            }
            Command::Right { extend } => {
                (None, sel::move_horizontal(rope, &self.sels, true, extend))
            }
            Command::Up { extend } => (None, sel::move_vertical(rope, &self.sels, false, extend)),
            Command::Down { extend } => (None, sel::move_vertical(rope, &self.sels, true, extend)),
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
