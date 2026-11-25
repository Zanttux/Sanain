//! Property: any sequence of valid edits, fully undone, restores the original
//! text exactly; fully redone, reproduces the edited text.

#![allow(clippy::unwrap_used)]

use proptest::prelude::*;
use sanain_core::{Change, Document, Transaction};

#[derive(Debug, Clone)]
struct Op {
    a: usize,
    b: usize,
    text: String,
}

fn op() -> impl Strategy<Value = Op> {
    (any::<usize>(), any::<usize>(), "[a-z\\r\\n é🙂漢]{0,6}").prop_map(|(a, b, text)| Op {
        a,
        b,
        text,
    })
}

proptest! {
    #[test]
    fn undo_all_restores_original(
        initial in "[a-z\\r\\n é🙂]{0,40}",
        ops in prop::collection::vec(op(), 0..40),
    ) {
        let mut doc = Document::from_saved_text(&initial);
        for o in &ops {
            let len = doc.len_chars();
            let (x, y) = (o.a % (len + 1), o.b % (len + 1));
            let range = x.min(y)..x.max(y);
            doc.apply(&Transaction::single(Change::replace(range, o.text.clone())))
                .unwrap();
        }
        let edited = doc.text();
        while doc.undo() {}
        prop_assert_eq!(doc.text(), initial);
        prop_assert!(!doc.is_dirty());
        while doc.redo() {}
        prop_assert_eq!(doc.text(), edited);
    }
}
