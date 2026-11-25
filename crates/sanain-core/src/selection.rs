//! Multi-cursor selections and the editing commands that act on them.
//!
//! Selections belong to a view, not the document, so two views of one
//! document can have different cursors. Commands produce a [`Transaction`]
//! plus the resulting selections; the caller applies both.

use std::ops::Range;

use ropey::Rope;

use crate::grapheme;
use crate::transaction::{Change, Transaction};

/// A selection from `anchor` to `head` (the caret), in char indices.
/// `anchor == head` is a plain cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    pub anchor: usize,
    pub head: usize,
}

impl Selection {
    pub fn cursor(at: usize) -> Self {
        Self {
            anchor: at,
            head: at,
        }
    }

    pub fn new(anchor: usize, head: usize) -> Self {
        Self { anchor, head }
    }

    pub fn range(&self) -> Range<usize> {
        self.anchor.min(self.head)..self.anchor.max(self.head)
    }

    pub fn is_empty(&self) -> bool {
        self.anchor == self.head
    }
}

/// A non-empty, sorted set of non-overlapping selections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selections {
    items: Vec<Selection>,
    primary: usize,
}

impl Selections {
    pub fn single(sel: Selection) -> Self {
        Self {
            items: vec![sel],
            primary: 0,
        }
    }

    /// Build from arbitrary selections: clamps to `len`, sorts, merges
    /// overlaps. `primary` indexes the input; an empty input yields a
    /// cursor at 0.
    pub fn new(items: Vec<Selection>, primary: usize, len: usize) -> Self {
        if items.is_empty() {
            return Self::single(Selection::cursor(0));
        }
        let primary_sel = items.get(primary).copied().unwrap_or(items[0]);
        let mut items: Vec<Selection> = items
            .into_iter()
            .map(|s| Selection::new(s.anchor.min(len), s.head.min(len)))
            .collect();
        items.sort_by_key(|s| (s.range().start, s.range().end));
        let mut merged: Vec<Selection> = Vec::with_capacity(items.len());
        for s in items {
            match merged.last_mut() {
                // Overlapping, or two cursors at the same spot.
                Some(last) if s.range().start < last.range().end || (s.range() == last.range()) => {
                    let r = last.range().start..last.range().end.max(s.range().end);
                    let forward = last.head >= last.anchor;
                    *last = if forward {
                        Selection::new(r.start, r.end)
                    } else {
                        Selection::new(r.end, r.start)
                    };
                }
                _ => merged.push(s),
            }
        }
        let p = primary_sel.anchor.min(len);
        let primary = merged
            .iter()
            .position(|s| s.range().contains(&p) || s.range().end == p)
            .unwrap_or(0);
        Self {
            items: merged,
            primary,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Selection> {
        self.items.iter()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn primary(&self) -> Selection {
        self.items[self.primary]
    }
}

/// Replace every selection with `text`; carets land after the insertion.
pub fn insert(sels: &Selections, text: &str) -> (Transaction, Selections) {
    replace_each(sels, |s| (s.range(), text.to_owned()))
}

/// Backspace: delete each selection, or the grapheme before each cursor.
pub fn delete_backward(rope: &Rope, sels: &Selections) -> (Transaction, Selections) {
    replace_each(sels, |s| {
        if s.is_empty() {
            (grapheme::prev_boundary(rope, s.head)..s.head, String::new())
        } else {
            (s.range(), String::new())
        }
    })
}

/// Delete: delete each selection, or the grapheme after each cursor.
pub fn delete_forward(rope: &Rope, sels: &Selections) -> (Transaction, Selections) {
    replace_each(sels, |s| {
        if s.is_empty() {
            (s.head..grapheme::next_boundary(rope, s.head), String::new())
        } else {
            (s.range(), String::new())
        }
    })
}

/// Move each caret one grapheme. With `extend`, keep anchors (Shift+Arrow);
/// without, a non-empty selection collapses to its edge in that direction.
pub fn move_horizontal(rope: &Rope, sels: &Selections, forward: bool, extend: bool) -> Selections {
    let moved = sels
        .iter()
        .map(|s| {
            if !extend && !s.is_empty() {
                let r = s.range();
                return Selection::cursor(if forward { r.end } else { r.start });
            }
            let head = if forward {
                grapheme::next_boundary(rope, s.head)
            } else {
                grapheme::prev_boundary(rope, s.head)
            };
            if extend {
                Selection::new(s.anchor, head)
            } else {
                Selection::cursor(head)
            }
        })
        .collect();
    Selections::new(moved, sels.primary, rope.len_chars())
}

/// Char index where `line`'s content ends, before its line break.
pub fn line_content_end(rope: &Rope, line: usize) -> usize {
    let start = rope.line_to_char(line);
    let slice = rope.line(line);
    let mut len = slice.len_chars();
    // Strip CRLF as a unit, or any single break char ropey recognises.
    if len >= 2 && slice.char(len - 2) == '\r' && slice.char(len - 1) == '\n' {
        len -= 2;
    } else if len >= 1
        && matches!(
            slice.char(len - 1),
            '\n' | '\r' | '\u{0B}' | '\u{0C}' | '\u{85}' | '\u{2028}' | '\u{2029}'
        )
    {
        len -= 1;
    }
    start + len
}

/// Zero-based (line, char column) of `char_idx`.
pub fn line_col(rope: &Rope, char_idx: usize) -> (usize, usize) {
    let idx = char_idx.min(rope.len_chars());
    let line = rope.char_to_line(idx);
    (line, idx - rope.line_to_char(line))
}

/// Up/Down by one line, keeping the char column where possible and never
/// landing inside a grapheme cluster. Past the first or last line the caret
/// goes to the document start or end.
pub fn move_vertical(rope: &Rope, sels: &Selections, down: bool, extend: bool) -> Selections {
    let last_line = rope.len_lines().saturating_sub(1);
    let moved = sels
        .iter()
        .map(|s| {
            let (line, col) = line_col(rope, s.head);
            let head = match (down, line) {
                (false, 0) => 0,
                (true, l) if l >= last_line => rope.len_chars(),
                _ => {
                    let target = if down { line + 1 } else { line - 1 };
                    let start = rope.line_to_char(target);
                    let mut at = (start + col).min(line_content_end(rope, target));
                    if !grapheme::is_boundary(rope, at) {
                        at = grapheme::prev_boundary(rope, at);
                    }
                    at
                }
            };
            if extend {
                Selection::new(s.anchor, head)
            } else {
                Selection::cursor(head)
            }
        })
        .collect();
    Selections::new(moved, sels.primary, rope.len_chars())
}

/// Home/End: move to the start or content end of the caret's line.
pub fn move_line_edge(rope: &Rope, sels: &Selections, end: bool, extend: bool) -> Selections {
    let moved = sels
        .iter()
        .map(|s| {
            let line = rope.char_to_line(s.head.min(rope.len_chars()));
            let head = if end {
                line_content_end(rope, line)
            } else {
                rope.line_to_char(line)
            };
            if extend {
                Selection::new(s.anchor, head)
            } else {
                Selection::cursor(head)
            }
        })
        .collect();
    Selections::new(moved, sels.primary, rope.len_chars())
}

pub fn select_all(rope: &Rope) -> Selections {
    Selections::single(Selection::new(0, rope.len_chars()))
}

/// The selected text of every non-empty selection, joined by `\n`.
pub fn selected_text(rope: &Rope, sels: &Selections) -> String {
    sels.iter()
        .filter(|s| !s.is_empty())
        .map(|s| rope.slice(s.range()).to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Apply one replacement per selection. Changes are emitted from the end of
/// the document backwards so each range is valid in original coordinates.
fn replace_each(
    sels: &Selections,
    mut f: impl FnMut(&Selection) -> (Range<usize>, String),
) -> (Transaction, Selections) {
    // Derived ranges can overlap even though selections do not: a cursor
    // right after a selection backspaces into it. Coalesce so every change
    // is valid and the user's intent (delete both) holds.
    let mut edits: Vec<(Range<usize>, String)> = Vec::with_capacity(sels.len());
    for (range, text) in sels.iter().map(&mut f) {
        match edits.last_mut() {
            Some((last, last_text)) if range.start < last.end => {
                last.start = last.start.min(range.start);
                last.end = last.end.max(range.end);
                last_text.push_str(&text);
            }
            _ => edits.push((range, text)),
        }
    }
    let mut tx = Transaction::new();
    for (range, text) in edits.iter().rev() {
        tx.push(Change::replace(range.clone(), text.clone()));
    }
    let mut delta: isize = 0;
    let mut carets = Vec::with_capacity(edits.len());
    for (range, text) in &edits {
        let inserted = text.chars().count();
        let start = range.start.saturating_add_signed(delta);
        carets.push(Selection::cursor(start + inserted));
        delta += inserted as isize - range.len() as isize;
    }
    // Carets are computed in post-edit coordinates; no clamping needed.
    (tx, Selections::new(carets, sels.primary, usize::MAX))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Document;

    fn run(
        text: &str,
        sels: Vec<Selection>,
        cmd: impl Fn(&Rope, &Selections) -> (Transaction, Selections),
    ) -> (String, Vec<Selection>) {
        let mut doc = Document::from_saved_text(text);
        let sels = Selections::new(sels, 0, doc.len_chars());
        let (tx, next) = cmd(doc.rope(), &sels);
        doc.apply(&tx).unwrap();
        (doc.text(), next.iter().copied().collect())
    }

    #[test]
    fn multi_cursor_insert() {
        let (text, sels) = run(
            "ab\ncd",
            vec![Selection::cursor(1), Selection::cursor(4)],
            |_, s| insert(s, "XY"),
        );
        assert_eq!(text, "aXYb\ncXYd");
        assert_eq!(sels, vec![Selection::cursor(3), Selection::cursor(8)]);
    }

    #[test]
    fn insert_replaces_selection() {
        let (text, sels) = run("hello world", vec![Selection::new(11, 6)], |_, s| {
            insert(s, "there")
        });
        assert_eq!(text, "hello there");
        assert_eq!(sels, vec![Selection::cursor(11)]);
    }

    #[test]
    fn backspace_removes_whole_grapheme_and_crlf() {
        let (text, sels) = run(
            "ae\u{301}\r\nb",
            vec![Selection::cursor(3), Selection::cursor(5)],
            delete_backward,
        );
        assert_eq!(text, "ab");
        assert_eq!(sels, vec![Selection::cursor(1)], "cursors merged");
        let (text, _) = run("x👨‍👩‍👧", vec![Selection::cursor(6)], delete_backward);
        assert_eq!(text, "x");
    }

    #[test]
    fn backspace_with_cursor_touching_selection_end() {
        // Selection 1..3 and a cursor at 3: both deletions overlap.
        let (text, sels) = run(
            "abcd",
            vec![Selection::new(1, 3), Selection::cursor(3)],
            delete_backward,
        );
        assert_eq!(text, "ad");
        assert_eq!(sels, vec![Selection::cursor(1)]);
    }

    #[test]
    fn backspace_at_start_and_delete_at_end_are_noops() {
        let (text, _) = run("ab", vec![Selection::cursor(0)], delete_backward);
        assert_eq!(text, "ab");
        let (text, _) = run("ab", vec![Selection::cursor(2)], delete_forward);
        assert_eq!(text, "ab");
    }

    #[test]
    fn delete_forward_multi_cursor() {
        let (text, sels) = run(
            "abcd",
            vec![Selection::cursor(0), Selection::cursor(2)],
            delete_forward,
        );
        assert_eq!(text, "bd");
        assert_eq!(sels, vec![Selection::cursor(0), Selection::cursor(1)]);
    }

    #[test]
    fn movement_by_grapheme_and_collapse() {
        let rope = Rope::from_str("ae\u{301}b");
        let s = Selections::single(Selection::cursor(1));
        let r = move_horizontal(&rope, &s, true, false);
        assert_eq!(r.primary(), Selection::cursor(3));
        let r = move_horizontal(&rope, &s, true, true);
        assert_eq!(r.primary(), Selection::new(1, 3));
        let sel = Selections::single(Selection::new(0, 3));
        assert_eq!(
            move_horizontal(&rope, &sel, false, false).primary(),
            Selection::cursor(0)
        );
        assert_eq!(
            move_horizontal(&rope, &sel, true, false).primary(),
            Selection::cursor(3)
        );
    }

    #[test]
    fn selections_merge_and_clamp() {
        let s = Selections::new(
            vec![
                Selection::new(5, 2),
                Selection::new(3, 8),
                Selection::cursor(50),
                Selection::cursor(50),
            ],
            0,
            10,
        );
        let v: Vec<_> = s.iter().copied().collect();
        assert_eq!(v, vec![Selection::new(8, 2), Selection::cursor(10)]);
        assert_eq!(s.primary(), Selection::new(8, 2));
    }

    #[test]
    fn vertical_movement_clamps_and_respects_crlf() {
        let rope = Rope::from_str("abcd\r\nx\r\nefgh");
        let s = Selections::single(Selection::cursor(3)); // line 0 col 3
        let d = move_vertical(&rope, &s, true, false);
        assert_eq!(
            d.primary(),
            Selection::cursor(7),
            "clamped before CRLF on 'x'"
        );
        let d2 = move_vertical(&rope, &d, true, false);
        assert_eq!(line_col(&rope, d2.primary().head), (2, 1));
        assert_eq!(
            move_vertical(&rope, &d2, true, false).primary(),
            Selection::cursor(13)
        );
        assert_eq!(
            move_vertical(&rope, &s, false, true).primary(),
            Selection::new(3, 0)
        );
    }

    #[test]
    fn vertical_movement_snaps_to_grapheme_boundary() {
        // Column 2 on line 1 falls between 'e' and its combining accent.
        let rope = Rope::from_str("abc\nxe\u{301}y");
        let s = Selections::single(Selection::cursor(2));
        assert_eq!(
            move_vertical(&rope, &s, true, false).primary(),
            Selection::cursor(5)
        );
    }

    #[test]
    fn home_end_and_select_all() {
        let rope = Rope::from_str("one\r\ntwo");
        let s = Selections::single(Selection::cursor(1));
        assert_eq!(
            move_line_edge(&rope, &s, true, false).primary(),
            Selection::cursor(3)
        );
        let s = Selections::single(Selection::cursor(7));
        assert_eq!(
            move_line_edge(&rope, &s, false, true).primary(),
            Selection::new(7, 5)
        );
        assert_eq!(select_all(&rope).primary(), Selection::new(0, 8));
        assert_eq!(selected_text(&rope, &select_all(&rope)), "one\r\ntwo");
    }

    #[test]
    fn line_content_end_handles_all_breaks() {
        let rope = Rope::from_str("a\nb\r\nc\rd");
        assert_eq!(line_content_end(&rope, 0), 1);
        assert_eq!(line_content_end(&rope, 1), 3);
        assert_eq!(line_content_end(&rope, 2), 6);
        assert_eq!(line_content_end(&rope, 3), 8);
    }
}
