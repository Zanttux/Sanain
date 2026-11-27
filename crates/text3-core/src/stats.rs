//! Character, word and line counts (the status bar's count item).
//!
//! The rules, chosen to match what the editor shows and moves by:
//!
//! - **Characters** are extended grapheme clusters (UAX #29), the unit the
//!   caret moves by: `e` + combining accent is one, a ZWJ emoji family is
//!   one, a CJK ideograph is one. Line breaks are not characters; a CRLF
//!   pair, LF, CR, VT, FF, NEL, LS and PS each end a line (the same breaks
//!   the editor numbers lines by).
//! - **Characters without spaces** leave out every cluster that starts
//!   with a whitespace character (spaces, tabs, no-break and ideographic
//!   spaces).
//! - **Words** are UAX #29 word segments that contain a letter or digit,
//!   so punctuation and spaces are not words and `don't` or `3.14` is one.
//!   Text written without spaces follows the same rules: each CJK
//!   ideograph and each hiragana character is a word of its own (as word
//!   processors count East Asian text), a katakana run is one word, Korean
//!   is split at spaces, and Thai, Lao, Khmer and Burmese runs without
//!   spaces count as one word each (no dictionary segmentation).
//! - **Lines**: a document has one line more than it has line breaks, as
//!   numbered in the gutter. A selection counts the lines it touches: a
//!   selection that ends right after a line break does not count the empty
//!   line after it.
//!
//! Counting walks the text line by line and checks `cancel` between lines,
//! so a caller on another thread can stop it early.

use std::borrow::Cow;
use std::ops::Range;

use ropey::{Rope, RopeSlice};
use unicode_segmentation::UnicodeSegmentation;

/// Counts of a document or of the selected text.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Counts {
    /// Grapheme clusters, line breaks excluded.
    pub chars: u64,
    /// Grapheme clusters that do not start with whitespace.
    pub non_space: u64,
    pub words: u64,
    pub lines: u64,
}

/// What one pass over a piece of text found.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Pass {
    chars: u64,
    non_space: u64,
    words: u64,
    breaks: u64,
    ends_with_break: bool,
}

/// Whether a grapheme cluster is a line break (ropey's set, CRLF as one).
fn is_break(g: &str) -> bool {
    matches!(
        g,
        "\r\n" | "\n" | "\r" | "\u{000B}" | "\u{000C}" | "\u{0085}" | "\u{2028}" | "\u{2029}"
    )
}

fn pass(slice: RopeSlice<'_>, cancel: &dyn Fn() -> bool) -> Option<Pass> {
    let mut p = Pass::default();
    for line in slice.lines() {
        if cancel() {
            return None;
        }
        if line.len_bytes() == 0 {
            continue;
        }
        let text: Cow<'_, str> = match line.as_str() {
            Some(s) => Cow::Borrowed(s),
            None => Cow::Owned(line.to_string()),
        };
        let mut last_break = false;
        for g in text.graphemes(true) {
            last_break = is_break(g);
            if last_break {
                p.breaks += 1;
                continue;
            }
            p.chars += 1;
            if !g.chars().next().is_some_and(char::is_whitespace) {
                p.non_space += 1;
            }
        }
        p.ends_with_break = last_break;
        p.words += text.unicode_words().count() as u64;
    }
    Some(p)
}

/// Counts for a whole document. `None` when `cancel` returned true.
pub fn document(rope: &Rope, cancel: &dyn Fn() -> bool) -> Option<Counts> {
    let p = pass(rope.slice(..), cancel)?;
    Some(Counts {
        chars: p.chars,
        non_space: p.non_space,
        words: p.words,
        lines: p.breaks + 1,
    })
}

/// Counts for selected char ranges, summed (multiple selections).
/// Empty ranges count nothing. `None` when `cancel` returned true.
pub fn ranges(rope: &Rope, ranges: &[Range<usize>], cancel: &dyn Fn() -> bool) -> Option<Counts> {
    let len = rope.len_chars();
    let mut out = Counts::default();
    for r in ranges {
        let (start, end) = (r.start.min(len), r.end.min(len));
        if start >= end {
            continue;
        }
        let p = pass(rope.slice(start..end), cancel)?;
        out.chars += p.chars;
        out.non_space += p.non_space;
        out.words += p.words;
        out.lines += p.breaks + u64::from(!p.ends_with_break);
    }
    Some(out)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::single_range_in_vec_init)]
mod tests {
    use super::*;

    fn never() -> bool {
        false
    }

    fn doc(text: &str) -> Counts {
        document(&Rope::from_str(text), &never).unwrap()
    }

    fn c(chars: u64, non_space: u64, words: u64, lines: u64) -> Counts {
        Counts {
            chars,
            non_space,
            words,
            lines,
        }
    }

    #[test]
    fn ascii() {
        assert_eq!(doc(""), c(0, 0, 0, 1));
        assert_eq!(doc("Hello, world!"), c(13, 12, 2, 1));
        assert_eq!(doc("one two\tthree\n"), c(13, 11, 3, 2));
        // Apostrophes and decimals stay inside words; punctuation alone is
        // not a word.
        assert_eq!(doc("don't stop 3.14 -- ok").words, 4);
    }

    #[test]
    fn combining_marks_and_emoji_are_one_character() {
        // e + COMBINING ACUTE ACCENT.
        assert_eq!(doc("cafe\u{301}"), c(4, 4, 1, 1));
        // Family: man ZWJ woman ZWJ girl; a flag; a skin-tone modifier.
        let family = "\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}";
        assert_eq!(doc(family).chars, 1);
        assert_eq!(doc("\u{1F1EB}\u{1F1EE}").chars, 1);
        assert_eq!(doc("\u{1F44D}\u{1F3FD}").chars, 1);
        // Emoji are not words.
        assert_eq!(doc(&format!("hi {family}")).words, 1);
    }

    #[test]
    fn cjk_counts_each_ideograph_as_a_word() {
        // 4 ideographs, an ideographic full stop (not a word).
        assert_eq!(doc("我爱北京。"), c(5, 5, 4, 1));
        // Hiragana one by one, katakana runs as one word.
        assert_eq!(doc("これはテストです").words, 6);
        // Korean words are separated by spaces; the ideographic space is
        // whitespace.
        assert_eq!(doc("안녕 하세요"), c(6, 5, 2, 1));
        assert_eq!(doc("日本\u{3000}語"), c(4, 3, 3, 1));
    }

    #[test]
    fn crlf_is_one_line_break() {
        assert_eq!(doc("a\r\nb\r\n"), c(2, 2, 2, 3));
        assert_eq!(doc("a\nb"), doc("a\r\nb"));
        assert_eq!(doc("a\rb").lines, 2);
        assert_eq!(doc("\r\n\r\n").chars, 0);
        // A CRLF that straddles rope chunks is still one break.
        let mut rope = Rope::from_str(&"x".repeat(5000));
        rope.insert(2500, "\r\n");
        let n = document(&rope, &never).unwrap();
        assert_eq!((n.chars, n.lines), (5000, 2));
    }

    #[test]
    fn selections_are_summed() {
        let rope = Rope::from_str("one two\nthree four\nfive\n");
        // "two" and "three four\n" (ends after a break: one line).
        let got = ranges(&rope, &[4..7, 8..19], &never).unwrap();
        assert_eq!(got, c(13, 12, 3, 2));
        // Empty ranges (carets) count nothing; out-of-range ends clamp.
        assert_eq!(ranges(&rope, &[3..3], &never).unwrap(), Counts::default());
        assert_eq!(ranges(&rope, &[19..999], &never).unwrap(), c(4, 4, 1, 1));
        // A selection across two lines without the final break: 2 lines.
        assert_eq!(ranges(&rope, &[0..12], &never).unwrap().lines, 2);
        // A word cut by the selection edge counts as a word.
        assert_eq!(ranges(&rope, &[1..2], &never).unwrap().words, 1);
    }

    #[test]
    fn cancel_stops_counting() {
        let rope = Rope::from_str("a\nb\n");
        assert_eq!(document(&rope, &|| true), None);
        assert_eq!(ranges(&rope, &[0..2], &|| true), None);
        assert_eq!(ranges(&rope, &[], &|| true), Some(Counts::default()));
    }
}
