//! Literal and regular-expression search over a document, and replace-all
//! as a single undoable transaction.
//!
//! Matching uses the `regex` crate, which guarantees linear-time matching
//! (no catastrophic backtracking), so hostile patterns cannot hang the UI.
//! Empty matches are skipped: they cannot be selected or meaningfully
//! replaced one by one.

use std::ops::Range;

use regex::{Regex, RegexBuilder};
use ropey::Rope;

use crate::transaction::{Change, Transaction};

/// Stop collecting after this many matches; the UI reports "N+".
pub const MATCH_LIMIT: usize = 100_000;
/// Upper bound on compiled pattern size (bytes), against pattern bombs.
const PATTERN_SIZE_LIMIT: usize = 1 << 20;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Query {
    pub pattern: String,
    pub regex: bool,
    pub case_sensitive: bool,
    pub whole_word: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SearchError {
    #[error("invalid pattern: {0}")]
    InvalidPattern(String),
}

#[derive(Debug, Clone)]
pub struct Matcher {
    re: Regex,
    regex_mode: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Matches {
    /// Char ranges, ascending, non-overlapping, non-empty.
    pub ranges: Vec<Range<usize>>,
    /// True when [`MATCH_LIMIT`] was reached.
    pub truncated: bool,
}

impl Query {
    pub fn compile(&self) -> Result<Option<Matcher>, SearchError> {
        if self.pattern.is_empty() {
            return Ok(None);
        }
        let core = if self.regex {
            self.pattern.clone()
        } else {
            regex::escape(&self.pattern)
        };
        let pattern = if self.whole_word {
            format!(r"\b(?:{core})\b")
        } else {
            core
        };
        let re = RegexBuilder::new(&pattern)
            .case_insensitive(!self.case_sensitive)
            .multi_line(true)
            .size_limit(PATTERN_SIZE_LIMIT)
            .build()
            .map_err(|e| SearchError::InvalidPattern(e.to_string()))?;
        Ok(Some(Matcher {
            re,
            regex_mode: self.regex,
        }))
    }
}

impl Matcher {
    pub fn find_all(&self, rope: &Rope) -> Matches {
        let text = rope.to_string();
        let mut out = Matches::default();
        // Walk byte offsets and convert to chars incrementally (matches are
        // ascending), avoiding a rope lookup per match.
        let (mut byte, mut chr) = (0usize, 0usize);
        for m in self.re.find_iter(&text) {
            if m.start() == m.end() {
                continue;
            }
            if out.ranges.len() == MATCH_LIMIT {
                out.truncated = true;
                break;
            }
            chr += text[byte..m.start()].chars().count();
            let start = chr;
            chr += m.as_str().chars().count();
            byte = m.end();
            out.ranges.push(start..chr);
        }
        out
    }

    /// Replace every match with `replacement` in one transaction. In regex
    /// mode, `$1` / `${name}` expand capture groups; otherwise the text is
    /// inserted literally. Returns the transaction and the number replaced.
    pub fn replace_all(&self, rope: &Rope, replacement: &str) -> (Transaction, usize) {
        let text = rope.to_string();
        let mut edits: Vec<(Range<usize>, String)> = Vec::new();
        let (mut byte, mut chr) = (0usize, 0usize);
        for caps in self.re.captures_iter(&text) {
            let Some(m) = caps.get(0) else { continue };
            if m.start() == m.end() {
                continue;
            }
            let mut with = String::new();
            if self.regex_mode {
                caps.expand(replacement, &mut with);
            } else {
                with.push_str(replacement);
            }
            chr += text[byte..m.start()].chars().count();
            let start = chr;
            chr += m.as_str().chars().count();
            byte = m.end();
            edits.push((start..chr, with));
        }
        let mut tx = Transaction::new();
        for (range, with) in edits.iter().rev() {
            tx.push(Change::replace(range.clone(), with.clone()));
        }
        (tx, edits.len())
    }

    /// The replacement for a single match at `range` (for "Replace").
    pub fn replacement_for(
        &self,
        rope: &Rope,
        range: Range<usize>,
        replacement: &str,
    ) -> Option<String> {
        let found: String = rope.slice(range).into();
        let caps = self.re.captures(&found)?;
        let m = caps.get(0)?;
        if m.start() != 0 || m.end() != found.len() {
            return None;
        }
        let mut with = String::new();
        if self.regex_mode {
            caps.expand(replacement, &mut with);
        } else {
            with.push_str(replacement);
        }
        Some(with)
    }
}

impl Matches {
    /// Index of the first match starting at or after `from` (forward), or
    /// the last match ending at or before `from` (backward), wrapping.
    pub fn next_from(&self, from: usize, forward: bool) -> Option<usize> {
        if self.ranges.is_empty() {
            return None;
        }
        if forward {
            Some(
                self.ranges
                    .iter()
                    .position(|r| r.start >= from)
                    .unwrap_or(0),
            )
        } else {
            Some(
                self.ranges
                    .iter()
                    .rposition(|r| r.end <= from)
                    .unwrap_or(self.ranges.len() - 1),
            )
        }
    }

    /// Index of the match exactly at `range`, if any.
    pub fn index_of(&self, range: &Range<usize>) -> Option<usize> {
        self.ranges
            .binary_search_by_key(&range.start, |r| r.start)
            .ok()
            .filter(|&i| self.ranges[i] == *range)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Document;

    fn q(pattern: &str) -> Query {
        Query {
            pattern: pattern.into(),
            ..Default::default()
        }
    }

    fn find(text: &str, query: Query) -> Vec<Range<usize>> {
        query
            .compile()
            .unwrap()
            .unwrap()
            .find_all(&Rope::from_str(text))
            .ranges
    }

    #[test]
    fn literal_is_escaped_and_case_insensitive_by_default() {
        assert_eq!(find("a.b A.B axb", q("a.b")), vec![0..3, 4..7]);
        let cs = Query {
            case_sensitive: true,
            ..q("a.b")
        };
        assert_eq!(find("a.b A.B", cs), vec![0..3]);
    }

    #[test]
    fn char_offsets_with_multibyte_text() {
        assert_eq!(find("🙂ä x 🙂ä", q("ä")), vec![1..2, 6..7]);
    }

    #[test]
    fn whole_word_and_regex() {
        let ww = Query {
            whole_word: true,
            ..q("cat")
        };
        assert_eq!(find("cat concat cat_ cat.", ww), vec![0..3, 16..19]);
        let re = Query {
            regex: true,
            ..q(r"^\w+")
        };
        assert_eq!(find("ab cd\r\nef", re), vec![0..2, 7..9]);
    }

    #[test]
    fn empty_pattern_and_empty_matches() {
        assert!(q("").compile().unwrap().is_none());
        let re = Query {
            regex: true,
            ..q("x*")
        };
        assert_eq!(find("axxb", re), vec![1..3]);
    }

    #[test]
    fn invalid_regex_is_an_error_not_a_panic() {
        let re = Query {
            regex: true,
            ..q("(unclosed")
        };
        assert!(matches!(re.compile(), Err(SearchError::InvalidPattern(_))));
    }

    #[test]
    fn replace_all_is_one_undoable_step_with_captures() {
        let mut doc = Document::from_saved_text("key=1\nother=22\n");
        let m = Query {
            regex: true,
            ..q(r"(\w+)=(\d+)")
        }
        .compile()
        .unwrap()
        .unwrap();
        let (tx, n) = m.replace_all(doc.rope(), "$2:$1");
        assert_eq!(n, 2);
        doc.apply(&tx).unwrap();
        assert_eq!(doc.text(), "1:key\n22:other\n");
        doc.undo();
        assert_eq!(doc.text(), "key=1\nother=22\n");
        // Literal mode inserts `$1` as-is.
        let lit = q("=").compile().unwrap().unwrap();
        let (tx, _) = lit.replace_all(doc.rope(), "$1");
        doc.apply(&tx).unwrap();
        assert_eq!(doc.text(), "key$11\nother$122\n");
    }

    #[test]
    fn navigation_wraps() {
        let m = Matches {
            ranges: vec![2..3, 5..6, 9..10],
            truncated: false,
        };
        assert_eq!(m.next_from(0, true), Some(0));
        assert_eq!(m.next_from(6, true), Some(2));
        assert_eq!(m.next_from(10, true), Some(0), "wraps forward");
        assert_eq!(m.next_from(5, false), Some(0));
        assert_eq!(m.next_from(1, false), Some(2), "wraps backward");
        assert_eq!(m.index_of(&(5..6)), Some(1));
        assert_eq!(m.index_of(&(5..7)), None);
    }

    #[test]
    fn pathological_pattern_is_fast() {
        // Classic catastrophic-backtracking pattern for backtracking engines.
        let text = "a".repeat(5_000) + "!";
        let re = Query {
            regex: true,
            ..q("(a+)+$")
        };
        let t = std::time::Instant::now();
        let _ = find(&text, re);
        assert!(t.elapsed() < std::time::Duration::from_secs(2));
    }
}
