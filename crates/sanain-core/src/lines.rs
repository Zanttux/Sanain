//! Whole-line operations over the lines touched by each selection:
//! delete, duplicate and move up/down. Each produces one transaction.

use std::ops::Range;

use ropey::Rope;

use crate::selection::{Selection, Selections, line_content_end};
use crate::transaction::{Change, Transaction};

/// Inclusive line spans covered by the selections, merged (overlapping or
/// adjacent) and ascending.
/// A selection ending at the start of a line does not include that line.
fn blocks(rope: &Rope, sels: &Selections) -> Vec<(usize, usize)> {
    let mut out: Vec<(usize, usize)> = Vec::new();
    for s in sels.iter() {
        let r = s.range();
        let first = rope.char_to_line(r.start);
        let mut last = rope.char_to_line(r.end);
        if last > first && rope.line_to_char(last) == r.end {
            last -= 1;
        }
        match out.last_mut() {
            // Adjacent blocks merge too: they act as one block, and their
            // edits would otherwise touch the same line break.
            Some(prev) if first <= prev.1 + 1 => prev.1 = prev.1.max(last),
            _ => out.push((first, last)),
        }
    }
    out
}

fn last_line(rope: &Rope) -> usize {
    rope.len_lines().saturating_sub(1)
}

/// Char range of lines `a..=b` including the final line break, if any.
fn span(rope: &Rope, a: usize, b: usize) -> Range<usize> {
    let end = if b >= last_line(rope) {
        rope.len_chars()
    } else {
        rope.line_to_char(b + 1)
    };
    rope.line_to_char(a)..end
}

fn text(rope: &Rope, r: &Range<usize>) -> String {
    rope.slice(r.clone()).to_string()
}

/// Split `s` into content and trailing line break.
fn split_break(s: &str) -> (&str, &str) {
    for br in [
        "\r\n", "\n", "\r", "\u{0B}", "\u{0C}", "\u{85}", "\u{2028}", "\u{2029}",
    ] {
        if let Some(c) = s.strip_suffix(br) {
            return (c, &s[c.len()..]);
        }
    }
    (s, "")
}

/// One edit per block plus how carets inside the block move.
struct BlockEdit {
    range: Range<usize>,
    text: String,
    /// Shift (in chars, may be negative) applied to carets in the block.
    shift: isize,
    /// Where carets go instead of shifting (used by delete).
    collapse_to: Option<usize>,
    block: Range<usize>,
}

fn apply_blocks(sels: &Selections, edits: Vec<BlockEdit>) -> (Transaction, Selections) {
    let mut tx = Transaction::new();
    for e in edits.iter().rev() {
        tx.push(Change::replace(e.range.clone(), e.text.clone()));
    }
    // Map each caret: shift by the net size change of earlier edits, plus
    // its own block's shift.
    let map = |p: usize| -> usize {
        let mut delta: isize = 0;
        for e in &edits {
            let in_block = p >= e.block.start && p <= e.block.end;
            if in_block {
                if let Some(c) = e.collapse_to {
                    return c.saturating_add_signed(delta);
                }
                // Keep carets inside the replaced region: a caret on the
                // block's end can otherwise overshoot when the trailing
                // line break changes hands.
                let new_end = e.range.start + e.text.chars().count();
                let local = p
                    .saturating_add_signed(e.shift)
                    .clamp(e.range.start, new_end);
                return local.saturating_add_signed(delta);
            }
            if p < e.range.start {
                break;
            }
            delta += e.text.chars().count() as isize - e.range.len() as isize;
        }
        p.saturating_add_signed(delta)
    };
    let next = sels
        .iter()
        .map(|s| Selection::new(map(s.anchor), map(s.head)))
        .collect();
    (tx, Selections::new(next, 0, usize::MAX))
}

pub fn delete_lines(rope: &Rope, sels: &Selections) -> (Transaction, Selections) {
    let len = rope.len_chars();
    let edits = blocks(rope, sels)
        .into_iter()
        .map(|(a, b)| {
            let block = span(rope, a, b);
            // Deleting the last line also removes the break before it.
            let range = if b >= last_line(rope) && a > 0 {
                line_content_end(rope, a - 1)..len
            } else {
                block.clone()
            };
            let collapse = if b >= last_line(rope) && a > 0 {
                rope.line_to_char(a - 1)
            } else {
                range.start
            };
            BlockEdit {
                range,
                text: String::new(),
                shift: 0,
                collapse_to: Some(collapse),
                block,
            }
        })
        .collect();
    apply_blocks(sels, edits)
}

/// Copy the touched lines below themselves; carets move into the copy.
pub fn duplicate_lines(rope: &Rope, sels: &Selections, newline: &str) -> (Transaction, Selections) {
    let edits = blocks(rope, sels)
        .into_iter()
        .map(|(a, b)| {
            let block = span(rope, a, b);
            let t = text(rope, &block);
            let (content, br) = split_break(&t);
            let (at, insert) = if br.is_empty() {
                (block.end, format!("{newline}{content}"))
            } else {
                (block.end, t.clone())
            };
            let shift = insert.chars().count() as isize;
            BlockEdit {
                range: at..at,
                text: insert,
                shift,
                collapse_to: None,
                block,
            }
        })
        .collect();
    apply_blocks(sels, edits)
}

/// Swap the touched lines with the line above (`down == false`) or below.
/// Blocks already at the document edge stay put.
pub fn move_lines(rope: &Rope, sels: &Selections, down: bool) -> (Transaction, Selections) {
    let last = last_line(rope);
    let edits = blocks(rope, sels)
        .into_iter()
        .filter(|&(a, b)| if down { b < last } else { a > 0 })
        .map(|(a, b)| {
            let block = span(rope, a, b);
            let bt = text(rope, &block);
            if down {
                let other = span(rope, b + 1, b + 1);
                let ot = text(rope, &other);
                let (oc, obr) = split_break(&ot);
                let new = if obr.is_empty() {
                    // The line below is the last line: it gains the block's
                    // break, the block loses it.
                    let (bc, bbr) = split_break(&bt);
                    format!("{oc}{bbr}{bc}")
                } else {
                    format!("{ot}{bt}")
                };
                let shift = if obr.is_empty() {
                    (oc.chars().count() + split_break(&bt).1.chars().count()) as isize
                } else {
                    ot.chars().count() as isize
                };
                BlockEdit {
                    range: block.start..other.end,
                    text: new,
                    shift,
                    collapse_to: None,
                    block,
                }
            } else {
                let other = span(rope, a - 1, a - 1);
                let ot = text(rope, &other);
                let (oc, obr) = split_break(&ot);
                let (bc, bbr) = split_break(&bt);
                let new = if bbr.is_empty() {
                    // The block is the last line: it gains the break.
                    format!("{bc}{obr}{oc}")
                } else {
                    format!("{bt}{ot}")
                };
                BlockEdit {
                    range: other.start..block.end,
                    text: new,
                    shift: -(ot.chars().count() as isize),
                    collapse_to: None,
                    block,
                }
            }
        })
        .collect();
    apply_blocks(sels, edits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Document;

    fn run(
        text: &str,
        sels: Vec<Selection>,
        f: impl Fn(&Rope, &Selections) -> (Transaction, Selections),
    ) -> (String, Vec<Selection>) {
        let mut doc = Document::from_saved_text(text);
        let s = Selections::new(sels, 0, doc.len_chars());
        let (tx, next) = f(doc.rope(), &s);
        doc.apply(&tx).unwrap();
        let out = doc.text();
        if out != text {
            assert!(doc.undo());
            assert_eq!(doc.text(), text, "undo restores");
        }
        (out, next.iter().copied().collect())
    }

    #[test]
    fn delete_middle_and_last_line() {
        let (t, s) = run("a\nbb\nc", vec![Selection::cursor(3)], delete_lines);
        assert_eq!(t, "a\nc");
        assert_eq!(s, vec![Selection::cursor(2)]);
        let (t, s) = run("a\r\nb", vec![Selection::cursor(3)], delete_lines);
        assert_eq!(t, "a");
        assert_eq!(s, vec![Selection::cursor(0)]);
        let (t, _) = run("only", vec![Selection::cursor(2)], delete_lines);
        assert_eq!(t, "");
    }

    #[test]
    fn duplicate_keeps_line_endings_and_moves_caret_into_copy() {
        let (t, s) = run("ab\r\ncd", vec![Selection::cursor(1)], |r, s| {
            duplicate_lines(r, s, "\r\n")
        });
        assert_eq!(t, "ab\r\nab\r\ncd");
        assert_eq!(s, vec![Selection::cursor(5)]);
        let (t, s) = run("ab\ncd", vec![Selection::cursor(4)], |r, s| {
            duplicate_lines(r, s, "\n")
        });
        assert_eq!(t, "ab\ncd\ncd");
        assert_eq!(s, vec![Selection::cursor(7)]);
    }

    #[test]
    fn move_down_and_up_including_last_line() {
        let (t, s) = run("a\nb\nc\n", vec![Selection::cursor(0)], |r, s| {
            move_lines(r, s, true)
        });
        assert_eq!(t, "b\na\nc\n");
        assert_eq!(s, vec![Selection::cursor(2)]);
        // Moving onto the last line without a trailing break.
        let (t, s) = run("a\r\nb", vec![Selection::cursor(1)], |r, s| {
            move_lines(r, s, true)
        });
        assert_eq!(t, "b\r\na");
        assert_eq!(s, vec![Selection::cursor(4)]);
        // Moving the last line up.
        let (t, s) = run("a\nb", vec![Selection::cursor(3)], |r, s| {
            move_lines(r, s, false)
        });
        assert_eq!(t, "b\na");
        assert_eq!(s, vec![Selection::cursor(1)]);
        // Edges are no-ops.
        let (t, _) = run("a\nb", vec![Selection::cursor(0)], |r, s| {
            move_lines(r, s, false)
        });
        assert_eq!(t, "a\nb");
    }

    #[test]
    fn multi_line_selection_moves_as_block() {
        // Lines "b" and "c" selected (ending at start of "d" excludes it).
        let (t, s) = run("a\nb\nc\nd", vec![Selection::new(2, 6)], |r, s| {
            move_lines(r, s, false)
        });
        assert_eq!(t, "b\nc\na\nd");
        assert_eq!(s, vec![Selection::new(0, 4)]);
    }

    #[test]
    fn multiple_cursors_on_separate_lines() {
        let (t, _) = run(
            "1\n2\n3\n4",
            vec![Selection::cursor(0), Selection::cursor(4)],
            |r, s| duplicate_lines(r, s, "\n"),
        );
        assert_eq!(t, "1\n1\n2\n3\n3\n4");
    }
}
