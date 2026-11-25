use std::ops::Range;

use ropey::Rope;

/// Replace the chars in `range` with `text`.
///
/// `range` uses char indices into the document *as it is when this change is
/// applied*, i.e. after all earlier changes of the same transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    pub range: Range<usize>,
    pub text: String,
}

impl Change {
    pub fn insert(at: usize, text: impl Into<String>) -> Self {
        Self {
            range: at..at,
            text: text.into(),
        }
    }

    pub fn delete(range: Range<usize>) -> Self {
        Self {
            range,
            text: String::new(),
        }
    }

    pub fn replace(range: Range<usize>, text: impl Into<String>) -> Self {
        Self {
            range,
            text: text.into(),
        }
    }
}

/// An ordered group of changes applied and undone as one step.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Transaction {
    changes: Vec<Change>,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EditError {
    #[error("range {start}..{end} is outside the document (length {len})")]
    OutOfBounds {
        start: usize,
        end: usize,
        len: usize,
    },
    #[error("range start {start} is after its end {end}")]
    InvertedRange { start: usize, end: usize },
}

impl Transaction {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn single(change: Change) -> Self {
        Self {
            changes: vec![change],
        }
    }

    pub fn push(&mut self, change: Change) -> &mut Self {
        self.changes.push(change);
        self
    }

    pub fn changes(&self) -> &[Change] {
        &self.changes
    }

    pub fn is_empty(&self) -> bool {
        self.changes
            .iter()
            .all(|c| c.range.start == c.range.end && c.text.is_empty())
    }

    /// Apply to `rope`, returning the exact inverse.
    ///
    /// Validation happens against a scratch copy first so a failing
    /// transaction never leaves the rope partially edited. Cloning a rope is
    /// O(1) (shared nodes).
    pub(crate) fn apply(&self, rope: &mut Rope) -> Result<Transaction, EditError> {
        let mut scratch = rope.clone();
        let mut inverse = Vec::with_capacity(self.changes.len());
        for change in &self.changes {
            let Range { start, end } = change.range;
            if start > end {
                return Err(EditError::InvertedRange { start, end });
            }
            let len = scratch.len_chars();
            if end > len {
                return Err(EditError::OutOfBounds { start, end, len });
            }
            let removed: String = scratch.slice(start..end).into();
            scratch.remove(start..end);
            scratch.insert(start, &change.text);
            inverse.push(Change {
                range: start..start + change.text.chars().count(),
                text: removed,
            });
        }
        *rope = scratch;
        inverse.reverse();
        Ok(Transaction { changes: inverse })
    }
}
