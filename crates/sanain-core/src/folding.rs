//! Indentation-based folding (language-agnostic), and the mapping between
//! document lines and display rows when some lines are folded away.
//!
//! A line starts a fold when the next non-blank line is indented deeper.
//! The fold hides the following lines up to (not including) the next
//! non-blank line indented at most as deep as the start line, minus
//! trailing blank lines. Closing brackets therefore stay visible.

use std::ops::Range;

use ropey::Rope;

/// Indentation width of `line` in columns (tabs to `tab` stops), or
/// `None` for a blank line.
pub fn indent(rope: &Rope, line: usize, tab: usize) -> Option<usize> {
    let mut col = 0;
    for c in rope.line(line).chars() {
        match c {
            ' ' => col += 1,
            '\t' => col += tab - col % tab,
            '\n' | '\r' => return None,
            _ => return Some(col),
        }
    }
    None
}

fn next_nonblank(rope: &Rope, from: usize, tab: usize) -> Option<(usize, usize)> {
    (from..rope.len_lines()).find_map(|l| indent(rope, l, tab).map(|i| (l, i)))
}

/// Cheap check used for gutter markers: does `line` start a fold?
pub fn is_fold_start(rope: &Rope, line: usize, tab: usize) -> bool {
    let Some(base) = indent(rope, line, tab) else {
        return false;
    };
    matches!(next_nonblank(rope, line + 1, tab), Some((_, i)) if i > base)
}

/// Lines hidden when folding at `line`, as a half-open range, or `None` if
/// the line does not start a fold.
pub fn fold_range(rope: &Rope, line: usize, tab: usize) -> Option<Range<usize>> {
    if !is_fold_start(rope, line, tab) {
        return None;
    }
    let base = indent(rope, line, tab)?;
    let total = rope.len_lines();
    let mut last_content = line;
    let mut l = line + 1;
    while l < total {
        match indent(rope, l, tab) {
            Some(i) if i <= base => break,
            Some(_) => last_content = l,
            None => {}
        }
        l += 1;
    }
    (last_content > line).then(|| line + 1..last_content + 1)
}

/// Which document lines are hidden, and how lines map to display rows.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FoldMap {
    /// Sorted, disjoint, non-empty half-open line ranges.
    hidden: Vec<Range<usize>>,
}

impl FoldMap {
    pub fn new(mut ranges: Vec<Range<usize>>) -> Self {
        ranges.retain(|r| !r.is_empty());
        ranges.sort_by_key(|r| r.start);
        let mut hidden: Vec<Range<usize>> = Vec::with_capacity(ranges.len());
        for r in ranges {
            match hidden.last_mut() {
                Some(last) if r.start <= last.end => last.end = last.end.max(r.end),
                _ => hidden.push(r),
            }
        }
        Self { hidden }
    }

    pub fn is_empty(&self) -> bool {
        self.hidden.is_empty()
    }

    pub fn hidden(&self) -> &[Range<usize>] {
        &self.hidden
    }

    pub fn is_hidden(&self, line: usize) -> bool {
        let i = self.hidden.partition_point(|r| r.end <= line);
        self.hidden.get(i).is_some_and(|r| r.contains(&line))
    }

    /// Number of display rows for a document of `lines` lines.
    pub fn rows(&self, lines: usize) -> usize {
        lines
            - self
                .hidden
                .iter()
                .map(|r| r.end.min(lines).saturating_sub(r.start))
                .sum::<usize>()
    }

    /// Display row of a visible `line` (hidden lines map to the row of the
    /// fold's start line).
    pub fn row_of_line(&self, line: usize) -> usize {
        let mut row = line;
        for r in &self.hidden {
            if r.start > line {
                break;
            }
            row -= (line.min(r.end) - r.start).min(r.end - r.start);
            if line < r.end {
                return row.saturating_sub(1);
            }
        }
        row
    }

    /// Document line shown at display `row`.
    pub fn line_of_row(&self, row: usize) -> usize {
        let mut line = row;
        for r in &self.hidden {
            if r.start <= line {
                line += r.end - r.start;
            } else {
                break;
            }
        }
        line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SRC: &str = "fn a() {\n    x;\n\n    if y {\n        z;\n    }\n}\nfn b() {}\n";

    #[test]
    fn indentation_folds() {
        let r = Rope::from_str(SRC);
        assert!(is_fold_start(&r, 0, 4));
        assert_eq!(
            fold_range(&r, 0, 4),
            Some(1..6),
            "closing brace stays visible"
        );
        assert_eq!(fold_range(&r, 3, 4), Some(4..5));
        assert_eq!(fold_range(&r, 1, 4), None);
        assert_eq!(fold_range(&r, 7, 4), None);
        let tabs = Rope::from_str("a:\n\tb\n\t\tc\nd\n");
        assert_eq!(fold_range(&tabs, 0, 4), Some(1..3));
        assert_eq!(fold_range(&tabs, 1, 4), Some(2..3));
    }

    #[test]
    fn trailing_blank_lines_stay_visible() {
        let r = Rope::from_str("a:\n  b\n\n\nc\n");
        assert_eq!(fold_range(&r, 0, 4), Some(1..2));
    }

    #[test]
    fn row_mapping_round_trips() {
        let m = FoldMap::new(vec![5..8, 2..4, 3..5]); // merges to 2..8
        assert_eq!(m.hidden(), std::slice::from_ref(&(2..8)));
        assert_eq!(m.rows(12), 6);
        let visible: Vec<usize> = (0..m.rows(12)).map(|row| m.line_of_row(row)).collect();
        assert_eq!(visible, vec![0, 1, 8, 9, 10, 11]);
        for (row, &line) in visible.iter().enumerate() {
            assert_eq!(m.row_of_line(line), row);
        }
        assert_eq!(m.row_of_line(5), 1, "hidden line maps to the fold's row");
        assert!(m.is_hidden(2) && m.is_hidden(7) && !m.is_hidden(8) && !m.is_hidden(1));
    }

    #[test]
    fn several_folds() {
        let m = FoldMap::new(vec![1..3, 6..9]);
        let visible: Vec<usize> = (0..m.rows(10)).map(|row| m.line_of_row(row)).collect();
        assert_eq!(visible, vec![0, 3, 4, 5, 9]);
        for (row, &line) in visible.iter().enumerate() {
            assert_eq!(m.row_of_line(line), row);
        }
    }
}
