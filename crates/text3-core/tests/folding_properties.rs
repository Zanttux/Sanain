//! Display-row mapping agrees with a naive model for random folds, and
//! mapping positions through a transaction matches applying it.

#![allow(clippy::unwrap_used)]

use proptest::prelude::*;
use text3_core::folding::FoldMap;
use text3_core::{Change, Document, Transaction};

proptest! {
    #[test]
    fn rows_match_naive(lines in 1usize..60, raw in prop::collection::vec((0usize..60, 1usize..10), 0..6)) {
        let ranges: Vec<_> = raw.iter().map(|&(s, len)| s.min(lines)..(s + len).min(lines)).collect();
        let m = FoldMap::new(ranges.clone());
        let visible: Vec<usize> = (0..lines).filter(|l| !ranges.iter().any(|r| r.contains(l))).collect();
        prop_assert_eq!(m.rows(lines), visible.len());
        for (row, &line) in visible.iter().enumerate() {
            prop_assert_eq!(m.line_of_row(row), line);
            prop_assert_eq!(m.row_of_line(line), row);
            prop_assert!(!m.is_hidden(line));
        }
    }

    #[test]
    fn map_pos_follows_text(text in "[a-c\\n]{0,30}", at in any::<usize>(), a in any::<usize>(), b in any::<usize>(), ins in "[x\\n]{0,4}") {
        let mut doc = Document::from_saved_text(&text);
        let len = doc.len_chars();
        let (a, b) = (a % (len + 1), b % (len + 1));
        let tx = Transaction::single(Change::replace(a.min(b)..a.max(b), ins.clone()));
        let pos = at % (len + 1);
        // A position before the edit keeps its index; after it, it keeps
        // pointing at the same character.
        let mapped = tx.map_pos(pos);
        doc.apply(&tx).unwrap();
        prop_assert!(mapped <= doc.len_chars());
        if pos >= a.max(b) && pos < len {
            let before: char = text.chars().nth(pos).unwrap();
            prop_assert_eq!(doc.rope().char(mapped), before);
        }
        if pos < a.min(b) {
            prop_assert_eq!(mapped, pos);
        }
    }
}
