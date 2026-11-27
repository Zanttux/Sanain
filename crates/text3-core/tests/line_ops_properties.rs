//! Line operations on random documents and selections: the transaction is
//! always valid, undo restores the original, and moving lines keeps the
//! same multiset of line contents.

#![allow(clippy::unwrap_used)]

use proptest::prelude::*;
use text3_core::lines::{delete_lines, duplicate_lines, move_lines};
use text3_core::{Document, Selection, Selections};

fn contents(text: &str) -> Vec<String> {
    let mut v: Vec<String> = text
        .split('\n')
        .map(|l| l.trim_end_matches('\r').to_owned())
        .collect();
    v.sort();
    v
}

proptest! {
    #[test]
    fn line_ops_are_valid_and_undoable(
        lines in prop::collection::vec("[a-c🙂]{0,3}", 1..8),
        crlf in any::<bool>(),
        trailing in any::<bool>(),
        picks in prop::collection::vec((any::<usize>(), any::<usize>()), 1..4),
        op in 0u8..4,
    ) {
        let br = if crlf { "\r\n" } else { "\n" };
        let mut text = lines.join(br);
        if trailing { text.push_str(br); }
        let mut doc = Document::from_saved_text(&text);
        let len = doc.len_chars();
        let sels = Selections::new(
            picks.iter().map(|&(a, b)| Selection::new(a % (len + 1), b % (len + 1))).collect(),
            0,
            len,
        );
        let (tx, next) = match op {
            0 => delete_lines(doc.rope(), &sels),
            1 => duplicate_lines(doc.rope(), &sels, br),
            2 => move_lines(doc.rope(), &sels, true),
            _ => move_lines(doc.rope(), &sels, false),
        };
        doc.apply(&tx).unwrap();
        let after = doc.text();
        for s in next.iter() {
            prop_assert!(s.head <= doc.len_chars() && s.anchor <= doc.len_chars());
        }
        if op >= 2 {
            prop_assert_eq!(contents(&after), contents(&text));
        }
        if after != text {
            prop_assert!(doc.undo());
        }
        prop_assert_eq!(doc.text(), text);
    }
}
