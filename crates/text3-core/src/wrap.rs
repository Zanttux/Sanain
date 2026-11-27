//! Soft wrapping: splitting long lines into display rows.
//!
//! Wrapping is computed here, not by the text renderer, so that rows,
//! hit testing and caret placement always agree. Widths are in monospace
//! columns: most characters take 1, East Asian wide characters 2, a tab
//! 4 (as the renderer draws it). Lines break after whitespace when
//! possible, otherwise anywhere. Whitespace may hang past the edge.

use std::ops::Range;

use ropey::Rope;
use unicode_width::UnicodeWidthChar;

use crate::folding::FoldMap;
use crate::selection::line_content_end;

fn width(c: char) -> usize {
    match c {
        '\t' => 4,
        c => c.width().unwrap_or(0),
    }
}

/// Char ranges of the display rows of one line (without its line break).
/// Always at least one range; an empty line gives `[0..0]`.
pub fn wrap_line(chars: impl Iterator<Item = char>, cols: usize) -> Vec<Range<usize>> {
    let cols = cols.max(1);
    let mut rows = Vec::new();
    let (mut start, mut col, mut i) = (0usize, 0usize, 0usize);
    // Char index just after the last whitespace in the current row.
    let mut brk: Option<(usize, usize)> = None; // (index, col at that point)
    for c in chars {
        let w = width(c);
        if c.is_whitespace() {
            col += w;
            i += 1;
            brk = Some((i, col));
            continue;
        }
        if col + w > cols && i > start {
            match brk.filter(|(b, _)| *b > start) {
                Some((b, bcol)) => {
                    rows.push(start..b);
                    start = b;
                    col -= bcol;
                }
                None => {
                    rows.push(start..i);
                    start = i;
                    col = 0;
                }
            }
            brk = None;
        }
        col += w;
        i += 1;
    }
    rows.push(start..i);
    rows
}

/// One display row: `start..end` chars of document line `line`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Row {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    /// The last row of its line (where the line break and fold marker go).
    pub last: bool,
}

/// Display rows of a document with folds and soft wrapping.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RowMap {
    rows: Vec<Row>,
    /// Per document line, the index of its first row (hidden lines: the
    /// first row of the line they are folded into).
    first: Vec<usize>,
}

impl RowMap {
    pub fn build(rope: &Rope, folds: &FoldMap, cols: usize) -> Self {
        let lines = rope.len_lines();
        let mut rows = Vec::with_capacity(lines);
        let mut first = Vec::with_capacity(lines);
        let mut last_visible = 0;
        for line in 0..lines {
            if folds.is_hidden(line) {
                first.push(last_visible);
                continue;
            }
            last_visible = rows.len();
            first.push(rows.len());
            let start = rope.line_to_char(line);
            let end = line_content_end(rope, line);
            let segs = wrap_line(rope.slice(start..end).chars(), cols);
            let n = segs.len();
            for (k, r) in segs.into_iter().enumerate() {
                rows.push(Row {
                    line,
                    start: r.start,
                    end: r.end,
                    last: k + 1 == n,
                });
            }
        }
        Self { rows, first }
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn row(&self, r: usize) -> Option<Row> {
        self.rows.get(r).copied()
    }

    /// The char index where the display row containing `head` starts or
    /// (with `to_end`) ends. A wrapped row ends before its wrap point
    /// (usually the space it broke after).
    pub fn row_edge(&self, rope: &Rope, head: usize, to_end: bool) -> usize {
        let line = rope.char_to_line(head.min(rope.len_chars()));
        let start = rope.line_to_char(line);
        let Some(row) = self.row(self.row_of(line, head - start)) else {
            return head;
        };
        match (to_end, row.last) {
            (false, _) => start + row.start,
            (true, true) => line_content_end(rope, line),
            (true, false) => (start + row.end).saturating_sub(1).max(start + row.start),
        }
    }

    /// The row showing column `col` (chars from the line start) of `line`.
    /// A position at a row boundary belongs to the later row, except at
    /// the end of the line.
    pub fn row_of(&self, line: usize, col: usize) -> usize {
        let Some(&f) = self.first.get(line) else {
            return self.rows.len().saturating_sub(1);
        };
        let mut r = f;
        while let Some(row) = self.rows.get(r) {
            if row.line != line || row.last || col < row.end {
                break;
            }
            r += 1;
        }
        r
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn row_edges() {
        // Rows at 10 columns: "aaaa bbbb " | "cccc dddd " | "ee"
        let rope = Rope::from_str("x\naaaa bbbb cccc dddd ee\n");
        let m = RowMap::build(&rope, &FoldMap::default(), 10);
        let at = |c: usize| 2 + c; // line 1 starts at char 2
        assert_eq!(m.row_edge(&rope, at(12), false), at(10));
        assert_eq!(m.row_edge(&rope, at(12), true), at(19), "before the space");
        assert_eq!(m.row_edge(&rope, at(10), false), at(10), "row start stays");
        assert_eq!(m.row_edge(&rope, at(3), true), at(9));
        assert_eq!(
            m.row_edge(&rope, at(21), true),
            at(22),
            "last row: line end"
        );
        assert_eq!(m.row_edge(&rope, at(21), false), at(20));
        assert_eq!(m.row_edge(&rope, 0, true), 1, "unwrapped line");
    }

    use super::*;

    fn wrap(s: &str, cols: usize) -> Vec<&str> {
        let chars: Vec<char> = s.chars().collect();
        wrap_line(s.chars(), cols)
            .into_iter()
            .map(|r| {
                let a: usize = chars[..r.start].iter().map(|c| c.len_utf8()).sum();
                let b: usize = chars[..r.end].iter().map(|c| c.len_utf8()).sum();
                &s[a..b]
            })
            .collect()
    }

    #[test]
    fn wraps_at_words_then_anywhere() {
        assert_eq!(wrap("", 10), vec![""]);
        assert_eq!(wrap("short", 10), vec!["short"]);
        assert_eq!(wrap("hello world again", 11), vec!["hello world ", "again"]);
        assert_eq!(
            wrap("hello world again", 8),
            vec!["hello ", "world ", "again"]
        );
        assert_eq!(wrap("abcdefghij", 4), vec!["abcd", "efgh", "ij"]);
        assert_eq!(wrap("a   b", 2), vec!["a   ", "b"], "spaces hang");
        // Wide characters take two columns.
        assert_eq!(wrap("日本語テキスト", 6), vec!["日本語", "テキス", "ト"]);
    }

    #[test]
    fn segments_cover_the_line_exactly() {
        let s = "The quick brown fox jumps over the lazy dog, again and again.";
        for cols in 1..70 {
            let rows = wrap_line(s.chars(), cols);
            assert_eq!(rows[0].start, 0);
            assert_eq!(rows.last().unwrap().end, s.chars().count());
            for w in rows.windows(2) {
                assert_eq!(w[0].end, w[1].start, "contiguous at {cols}");
                assert!(w[0].end > w[0].start, "non-empty at {cols}");
            }
        }
    }

    #[test]
    fn row_map_with_folds() {
        let rope = Rope::from_str("aaaa bbbb\nx\n  y\n  z\nlast line here\n");
        let folds = FoldMap::new(std::iter::once(3..4).collect());
        let m = RowMap::build(&rope, &folds, 5);
        let rows: Vec<(usize, usize, usize, bool)> = (0..m.len())
            .map(|r| m.row(r).map(|r| (r.line, r.start, r.end, r.last)).unwrap())
            .collect();
        assert_eq!(
            rows,
            vec![
                (0, 0, 5, false),
                (0, 5, 9, true),
                (1, 0, 1, true),
                (2, 0, 3, true),
                (4, 0, 5, false),
                (4, 5, 10, false),
                (4, 10, 14, true),
                (5, 0, 0, true),
            ]
        );
        assert_eq!(m.row_of(0, 4), 0);
        assert_eq!(m.row_of(0, 5), 1, "boundary belongs to the next row");
        assert_eq!(m.row_of(0, 9), 1, "line end stays on the last row");
        assert_eq!(m.row_of(3, 0), 3, "hidden line maps to its fold's row");
        assert_eq!(m.row_of(4, 14), 6);
    }
}
