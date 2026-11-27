//! Line diff for side-by-side comparison, with hunks that can be applied.
//!
//! Lines include their terminator (`\n` or `\r\n`), so a line-ending change
//! is a real difference and applying a hunk reproduces the other side's
//! bytes exactly. Diffing is time-bounded: very large inputs fall back to a
//! coarser (still correct) result rather than freezing the editor.

use std::ops::Range;
use std::time::{Duration, Instant};

use similar::{Algorithm, DiffOp, capture_diff_slices_deadline};

/// One display row of the side-by-side view (line indices).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Row {
    Same {
        left: usize,
        right: usize,
    },
    /// Changed line present on both sides.
    Changed {
        left: usize,
        right: usize,
    },
    LeftOnly(usize),
    RightOnly(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hunk {
    /// Line ranges on each side.
    pub left_lines: Range<usize>,
    pub right_lines: Range<usize>,
    /// Byte ranges on each side, for applying the hunk.
    pub left_bytes: Range<usize>,
    pub right_bytes: Range<usize>,
    /// Display rows belonging to this hunk.
    pub rows: Range<usize>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Diff {
    pub rows: Vec<Row>,
    pub hunks: Vec<Hunk>,
}

/// Byte offset where each line starts, plus the total length at the end.
fn line_starts(lines: &[&str]) -> Vec<usize> {
    let mut v = Vec::with_capacity(lines.len() + 1);
    let mut at = 0;
    v.push(0);
    for l in lines {
        at += l.len();
        v.push(at);
    }
    v
}

pub fn split_lines(text: &str) -> Vec<&str> {
    text.split_inclusive('\n').collect()
}

pub fn diff_lines(left: &str, right: &str) -> Diff {
    diff_lines_with(left, right, false)
}

/// Like [`diff_lines`]; with `ignore_whitespace`, lines that differ only in
/// whitespace (including line endings) count as equal. Hunk byte ranges
/// still refer to the original text, so applying a hunk stays exact.
pub fn diff_lines_with(left: &str, right: &str, ignore_whitespace: bool) -> Diff {
    let a = split_lines(left);
    let b = split_lines(right);
    let (sa, sb) = (line_starts(&a), line_starts(&b));
    let deadline = Some(Instant::now() + Duration::from_secs(2));
    let ops = if ignore_whitespace {
        let key = |l: &&str| l.chars().filter(|c| !c.is_whitespace()).collect::<String>();
        let ka: Vec<String> = a.iter().map(key).collect();
        let kb: Vec<String> = b.iter().map(key).collect();
        capture_diff_slices_deadline(Algorithm::Myers, &ka, &kb, deadline)
    } else {
        capture_diff_slices_deadline(Algorithm::Myers, &a, &b, deadline)
    };
    let mut diff = Diff::default();
    for op in ops {
        let (oi, ol, ni, nl) = match op {
            DiffOp::Equal {
                old_index,
                new_index,
                len,
            } => {
                for k in 0..len {
                    diff.rows.push(Row::Same {
                        left: old_index + k,
                        right: new_index + k,
                    });
                }
                continue;
            }
            DiffOp::Delete {
                old_index,
                old_len,
                new_index,
            } => (old_index, old_len, new_index, 0),
            DiffOp::Insert {
                old_index,
                new_index,
                new_len,
            } => (old_index, 0, new_index, new_len),
            DiffOp::Replace {
                old_index,
                old_len,
                new_index,
                new_len,
            } => (old_index, old_len, new_index, new_len),
        };
        let first_row = diff.rows.len();
        for k in 0..ol.max(nl) {
            diff.rows.push(match (k < ol, k < nl) {
                (true, true) => Row::Changed {
                    left: oi + k,
                    right: ni + k,
                },
                (true, false) => Row::LeftOnly(oi + k),
                _ => Row::RightOnly(ni + k),
            });
        }
        let hunk = Hunk {
            left_lines: oi..oi + ol,
            right_lines: ni..ni + nl,
            left_bytes: sa[oi]..sa[oi + ol],
            right_bytes: sb[ni]..sb[ni + nl],
            rows: first_row..diff.rows.len(),
        };
        // Adjacent change ops form one hunk.
        match diff.hunks.last_mut() {
            Some(h) if h.rows.end == first_row => {
                h.left_lines.end = hunk.left_lines.end;
                h.right_lines.end = hunk.right_lines.end;
                h.left_bytes.end = hunk.left_bytes.end;
                h.right_bytes.end = hunk.right_bytes.end;
                h.rows.end = hunk.rows.end;
            }
            _ => diff.hunks.push(hunk),
        }
    }
    diff
}

/// Make `left` match `right` within `hunk`: returns the left byte range to
/// replace and the replacement text.
pub fn take_right<'r>(hunk: &Hunk, right: &'r str) -> (Range<usize>, &'r str) {
    (hunk.left_bytes.clone(), &right[hunk.right_bytes.clone()])
}

/// Lines longer than this are not compared character by character.
const INTRALINE_MAX: usize = 2_000;

/// Changed character ranges within a pair of changed lines, for
/// highlighting: (ranges in `left`, ranges in `right`), in chars. Empty
/// when the lines are too long or too different for the detail to help
/// (less than 40 % in common).
pub fn intraline(left: &str, right: &str) -> (Vec<Range<usize>>, Vec<Range<usize>>) {
    let (a, b): (Vec<char>, Vec<char>) = (left.chars().collect(), right.chars().collect());
    if a.len() > INTRALINE_MAX || b.len() > INTRALINE_MAX {
        return (Vec::new(), Vec::new());
    }
    let ops = capture_diff_slices_deadline(
        Algorithm::Myers,
        &a,
        &b,
        Some(Instant::now() + Duration::from_millis(20)),
    );
    let same: usize = ops
        .iter()
        .map(|op| match op {
            DiffOp::Equal { len, .. } => *len,
            _ => 0,
        })
        .sum();
    if same * 10 < a.len().max(b.len()) * 4 {
        return (Vec::new(), Vec::new());
    }
    let (mut l, mut r) = (Vec::new(), Vec::new());
    let push = |v: &mut Vec<Range<usize>>, range: Range<usize>| {
        if range.is_empty() {
            return;
        }
        match v.last_mut() {
            Some(last) if last.end == range.start => last.end = range.end,
            _ => v.push(range),
        }
    };
    for op in ops {
        match op {
            DiffOp::Equal { .. } => {}
            DiffOp::Delete {
                old_index, old_len, ..
            } => push(&mut l, old_index..old_index + old_len),
            DiffOp::Insert {
                new_index, new_len, ..
            } => push(&mut r, new_index..new_index + new_len),
            DiffOp::Replace {
                old_index,
                old_len,
                new_index,
                new_len,
            } => {
                push(&mut l, old_index..old_index + old_len);
                push(&mut r, new_index..new_index + new_len);
            }
        }
    }
    (l, r)
}

#[cfg(test)]
mod tests {

    #[test]
    fn ignore_whitespace() {
        let l = "fn a() {\n    x();\n}\r\nend\n";
        let r = "fn a() {\n\tx( );\n}\nend changed\n";
        assert_eq!(
            diff_lines(l, r).hunks.len(),
            1,
            "all whitespace edits are one hunk with the real change"
        );
        assert!(diff_lines(l, r).hunks[0].left_lines.start == 1);
        let d = diff_lines_with(l, r, true);
        assert_eq!(d.hunks.len(), 1);
        assert_eq!(d.hunks[0].left_lines, 3..4, "only the real change remains");
        assert_eq!(&l[d.hunks[0].left_bytes.clone()], "end\n");
        assert_eq!(&r[d.hunks[0].right_bytes.clone()], "end changed\n");
        assert!(diff_lines_with("a b\n", "ab\r\n", true).hunks.is_empty());
    }
    use super::*;

    fn apply_all(left: &str, right: &str) -> String {
        let d = diff_lines(left, right);
        let mut out = left.to_owned();
        for h in d.hunks.iter().rev() {
            let (r, t) = take_right(h, right);
            out.replace_range(r, t);
        }
        out
    }

    #[test]
    fn intraline_marks_only_the_changed_part() {
        let (l, r) = intraline("let a = 1;", "let a = 10;");
        assert_eq!(l, Vec::<Range<usize>>::new());
        assert_eq!(r, vec![9..10]);
        let (l, r) = intraline("println!(\"hi\")", "eprintln!(\"ho\")");
        assert_eq!(l, vec![11..12]);
        assert_eq!(r, vec![0..1, 12..13]);
        let (l, r) = intraline("äö x", "äö y");
        assert_eq!(l, std::slice::from_ref(&(3..4)), "char indices, not bytes");
        assert_eq!(r, std::slice::from_ref(&(3..4)));
        // Unrelated lines: no detail.
        assert_eq!(intraline("abcdef", "uvwxyz"), (vec![], vec![]));
    }

    #[test]
    fn rows_and_hunks() {
        let d = diff_lines("a\nb\nc\nd\n", "a\nB\nc\nd\ne\n");
        assert_eq!(
            d.rows,
            vec![
                Row::Same { left: 0, right: 0 },
                Row::Changed { left: 1, right: 1 },
                Row::Same { left: 2, right: 2 },
                Row::Same { left: 3, right: 3 },
                Row::RightOnly(4),
            ]
        );
        assert_eq!(d.hunks.len(), 2);
        assert_eq!(d.hunks[0].left_bytes, 2..4);
        assert_eq!(d.hunks[1].left_bytes, 8..8);
    }

    #[test]
    fn identical_and_empty() {
        assert!(diff_lines("x\ny", "x\ny").hunks.is_empty());
        assert_eq!(diff_lines("", "").rows, vec![]);
        assert_eq!(apply_all("", "new\n"), "new\n");
        assert_eq!(apply_all("old\n", ""), "");
    }

    #[test]
    fn line_endings_and_missing_final_newline_are_exact() {
        assert_eq!(diff_lines("a\r\nb\r\n", "a\nb\n").hunks.len(), 1);
        assert_eq!(apply_all("a\r\nb\r\n", "a\nb"), "a\nb");
        assert_eq!(apply_all("a\nb", "a\nb\nc\n"), "a\nb\nc\n");
    }

    proptest::proptest! {
        #[test]
        fn applying_every_hunk_yields_right(
            l in prop::collection::vec("[abc]{0,2}(\\n|\\r\\n)?", 0..12),
            r in prop::collection::vec("[abc]{0,2}(\\n|\\r\\n)?", 0..12),
        ) {
            let (l, r) = (l.concat(), r.concat());
            proptest::prop_assert_eq!(apply_all(&l, &r), r);
        }
    }

    use proptest::prelude::prop;
}
