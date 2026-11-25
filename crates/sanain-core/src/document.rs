use ropey::Rope;

use crate::transaction::{EditError, Transaction};

/// Monotonic edit counter. Increases on every applied edit, undo and redo;
/// background work tagged with an older revision is stale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Revision(pub u64);

/// Line-ending style observed in a text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    /// No line breaks.
    None,
    Lf,
    CrLf,
    Cr,
    /// More than one style is present; the text is kept as-is.
    Mixed,
}

impl LineEnding {
    pub fn detect(text: &str) -> Self {
        let bytes = text.as_bytes();
        let (mut lf, mut crlf, mut cr) = (false, false, false);
        let mut i = 0;
        while i < bytes.len() {
            match bytes[i] {
                b'\r' if bytes.get(i + 1) == Some(&b'\n') => {
                    crlf = true;
                    i += 1;
                }
                b'\r' => cr = true,
                b'\n' => lf = true,
                _ => {}
            }
            i += 1;
        }
        match (lf, crlf, cr) {
            (false, false, false) => Self::None,
            (true, false, false) => Self::Lf,
            (false, true, false) => Self::CrLf,
            (false, false, true) => Self::Cr,
            _ => Self::Mixed,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Document {
    rope: Rope,
    revision: Revision,
    /// History entries are tagged with unique ids. The id of the newest
    /// undo entry identifies the current content state in history.
    undo: Vec<(u64, Transaction)>,
    redo: Vec<(u64, Transaction)>,
    next_id: u64,
    /// History position (top undo id, `None` = empty history) of the last
    /// clean state: the loaded file, the last save, or a fresh empty buffer.
    saved: Option<u64>,
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

impl Document {
    /// An empty document. It is clean until edited.
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            revision: Revision(0),
            undo: Vec::new(),
            redo: Vec::new(),
            next_id: 0,
            saved: None,
        }
    }

    /// A document whose content matches a file just loaded from disk.
    pub fn from_saved_text(text: &str) -> Self {
        let mut doc = Self::new();
        doc.rope = Rope::from_str(text);
        doc
    }

    pub fn text(&self) -> String {
        self.rope.to_string()
    }

    pub fn rope(&self) -> &Rope {
        &self.rope
    }

    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    pub fn revision(&self) -> Revision {
        self.revision
    }

    /// True when content differs from the last save, judged by history
    /// position: undoing back to the saved state makes the document clean.
    pub fn is_dirty(&self) -> bool {
        self.saved != self.position()
    }

    fn position(&self) -> Option<u64> {
        self.undo.last().map(|(id, _)| *id)
    }

    /// Record that the current content was written to disk successfully.
    /// Call only after the save has completed.
    pub fn mark_saved(&mut self) {
        self.saved = self.position();
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    /// Apply `tx` as one undoable step. Empty transactions are ignored.
    pub fn apply(&mut self, tx: &Transaction) -> Result<(), EditError> {
        if tx.is_empty() {
            return Ok(());
        }
        let inverse = tx.apply(&mut self.rope)?;
        let id = self.next_id;
        self.next_id += 1;
        self.undo.push((id, inverse));
        self.redo.clear();
        self.bump();
        Ok(())
    }

    pub fn undo(&mut self) -> bool {
        self.step(false).is_some()
    }

    pub fn redo(&mut self) -> bool {
        self.step(true).is_some()
    }

    /// Undo and return where carets belong afterwards, or `None` if there
    /// was nothing to undo.
    pub fn undo_carets(&mut self) -> Option<Vec<usize>> {
        self.step(false)
    }

    pub fn redo_carets(&mut self) -> Option<Vec<usize>> {
        self.step(true)
    }

    fn step(&mut self, redo: bool) -> Option<Vec<usize>> {
        let (from, to) = if redo {
            (&mut self.redo, &mut self.undo)
        } else {
            (&mut self.undo, &mut self.redo)
        };
        let (id, tx) = from.pop()?;
        let carets = tx.caret_positions();
        // History entries were produced from valid edits of this exact rope
        // state, so applying them cannot fail; keep the entry on error anyway.
        match tx.apply(&mut self.rope) {
            Ok(inverse) => {
                to.push((id, inverse));
                self.bump();
                Some(carets)
            }
            Err(_) => {
                from.push((id, tx));
                None
            }
        }
    }

    fn bump(&mut self) {
        self.revision = Revision(self.revision.0 + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Change;

    #[test]
    fn insert_undo_redo() {
        let mut doc = Document::from_saved_text("hello");
        doc.apply(&Transaction::single(Change::insert(5, " world")))
            .unwrap();
        assert_eq!(doc.text(), "hello world");
        assert!(doc.is_dirty());
        assert!(doc.undo());
        assert_eq!(doc.text(), "hello");
        assert!(doc.redo());
        assert_eq!(doc.text(), "hello world");
        assert!(!doc.redo());
    }

    #[test]
    fn multi_change_transaction_undoes_as_one_step() {
        let mut doc = Document::from_saved_text("abc");
        let mut tx = Transaction::new();
        tx.push(Change::insert(0, "1"))
            .push(Change::replace(2..3, "B"))
            .push(Change::delete(3..4));
        doc.apply(&tx).unwrap();
        assert_eq!(doc.text(), "1aB");
        assert!(doc.undo());
        assert_eq!(doc.text(), "abc");
        assert!(!doc.can_undo());
    }

    #[test]
    fn invalid_transaction_leaves_document_untouched() {
        let mut doc = Document::from_saved_text("abc");
        let mut tx = Transaction::new();
        tx.push(Change::insert(0, "x")).push(Change::delete(2..99));
        let err = doc.apply(&tx).unwrap_err();
        assert!(matches!(err, EditError::OutOfBounds { len: 4, .. }));
        assert_eq!(doc.text(), "abc");
        assert_eq!(doc.revision(), Revision(0));
        assert!(!doc.can_undo());
    }

    #[test]
    #[allow(clippy::reversed_empty_ranges)]
    fn inverted_range_is_rejected() {
        let mut doc = Document::from_saved_text("abc");
        let err = doc
            .apply(&Transaction::single(Change::delete(2..1)))
            .unwrap_err();
        assert_eq!(err, EditError::InvertedRange { start: 2, end: 1 });
    }

    #[test]
    fn new_edit_clears_redo() {
        let mut doc = Document::new();
        doc.apply(&Transaction::single(Change::insert(0, "a")))
            .unwrap();
        doc.undo();
        doc.apply(&Transaction::single(Change::insert(0, "b")))
            .unwrap();
        assert!(!doc.can_redo());
    }

    #[test]
    fn revision_increases_on_every_mutation() {
        let mut doc = Document::new();
        let r0 = doc.revision();
        doc.apply(&Transaction::single(Change::insert(0, "a")))
            .unwrap();
        let r1 = doc.revision();
        doc.undo();
        let r2 = doc.revision();
        assert!(r0 < r1 && r1 < r2);
    }

    #[test]
    fn empty_transaction_is_noop() {
        let mut doc = Document::from_saved_text("x");
        doc.apply(&Transaction::new()).unwrap();
        doc.apply(&Transaction::single(Change::insert(1, "")))
            .unwrap();
        assert!(!doc.is_dirty());
        assert!(!doc.can_undo());
    }

    #[test]
    fn undo_to_saved_point_is_clean_and_redo_is_dirty() {
        let mut doc = Document::from_saved_text("a");
        doc.apply(&Transaction::single(Change::insert(1, "b")))
            .unwrap();
        doc.mark_saved();
        doc.apply(&Transaction::single(Change::insert(2, "c")))
            .unwrap();
        assert!(doc.is_dirty());
        doc.undo();
        assert!(!doc.is_dirty());
        doc.undo();
        assert!(doc.is_dirty());
        doc.redo();
        assert!(!doc.is_dirty());
    }

    #[test]
    fn unreachable_save_point_stays_dirty() {
        let mut doc = Document::from_saved_text("a");
        doc.apply(&Transaction::single(Change::insert(1, "b")))
            .unwrap();
        doc.mark_saved();
        doc.undo();
        // Branching history discards the saved state from redo.
        doc.apply(&Transaction::single(Change::insert(1, "b")))
            .unwrap();
        assert_eq!(doc.text(), "ab");
        assert!(doc.is_dirty(), "content equal but save point unreachable");
    }

    #[test]
    fn new_document_is_clean_until_edited() {
        let mut doc = Document::new();
        assert!(!doc.is_dirty());
        doc.apply(&Transaction::single(Change::insert(0, "a")))
            .unwrap();
        assert!(doc.is_dirty());
        doc.mark_saved();
        assert!(!doc.is_dirty());
    }

    #[test]
    fn edits_preserve_crlf_and_non_ascii() {
        let mut doc = Document::from_saved_text("a\r\n🙂é\r\n");
        // Char indices: a=0 \r=1 \n=2 🙂=3 é=4
        doc.apply(&Transaction::single(Change::insert(4, "x")))
            .unwrap();
        assert_eq!(doc.text(), "a\r\n🙂xé\r\n");
        doc.undo();
        assert_eq!(doc.text(), "a\r\n🙂é\r\n");
    }

    #[test]
    fn undo_reports_caret_positions() {
        let mut doc = Document::from_saved_text("abcd");
        let mut tx = Transaction::new();
        // Descending order, as multi-cursor commands emit.
        tx.push(Change::insert(3, "XX"))
            .push(Change::insert(1, "Y"));
        doc.apply(&tx).unwrap();
        assert_eq!(doc.text(), "aYbcXXd");
        assert_eq!(doc.undo_carets(), Some(vec![1, 3]));
        assert_eq!(doc.redo_carets(), Some(vec![2, 6]));
        assert_eq!(doc.redo_carets(), None);
    }

    #[test]
    fn line_ending_detection() {
        assert_eq!(LineEnding::detect(""), LineEnding::None);
        assert_eq!(LineEnding::detect("a\nb\n"), LineEnding::Lf);
        assert_eq!(LineEnding::detect("a\r\nb\r\n"), LineEnding::CrLf);
        assert_eq!(LineEnding::detect("a\rb"), LineEnding::Cr);
        assert_eq!(LineEnding::detect("a\r\nb\n"), LineEnding::Mixed);
    }
}
