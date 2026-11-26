//! Pure, deterministic text transformations. Every function takes text and
//! returns new text (or an error); the app applies the result as one
//! undoable edit. Nothing here touches the network or the filesystem.

pub mod csv;
pub mod diff;
pub mod json;
pub mod markup;
pub mod xml;

/// Which line ending to convert to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Newline {
    Lf,
    CrLf,
}

/// Split into lines, keeping each line's own terminator separately.
fn lines_with_breaks(text: &str) -> Vec<(&str, &str)> {
    let mut out = Vec::new();
    let mut rest = text;
    while !rest.is_empty() {
        match rest.find(['\n', '\r']) {
            Some(i) => {
                let brk = if rest[i..].starts_with("\r\n") { 2 } else { 1 };
                out.push((&rest[..i], &rest[i..i + brk]));
                rest = &rest[i + brk..];
            }
            None => {
                out.push((rest, ""));
                rest = "";
            }
        }
    }
    out
}

/// The break used between lines when lines move around: the first one in
/// the text, or `\n` when there is none.
fn dominant_break(text: &str) -> &str {
    lines_with_breaks(text)
        .into_iter()
        .map(|(_, b)| b)
        .find(|b| !b.is_empty())
        .unwrap_or("\n")
}

/// Rebuild text from line contents, keeping whether it ended with a break.
fn join(lines: &[&str], brk: &str, trailing: bool) -> String {
    let mut s = lines.join(brk);
    if trailing && !lines.is_empty() {
        s.push_str(brk);
    }
    s
}

fn ends_with_break(text: &str) -> bool {
    text.ends_with('\n') || text.ends_with('\r')
}

/// Sort lines by Unicode scalar order (stable; case-sensitive).
pub fn sort_lines(text: &str, descending: bool) -> String {
    let mut lines: Vec<&str> = lines_with_breaks(text)
        .into_iter()
        .map(|(l, _)| l)
        .collect();
    lines.sort();
    if descending {
        lines.reverse();
    }
    join(&lines, dominant_break(text), ends_with_break(text))
}

/// Remove repeated lines, keeping the first occurrence and the order.
pub fn unique_lines(text: &str) -> String {
    let mut seen = std::collections::HashSet::new();
    let lines: Vec<&str> = lines_with_breaks(text)
        .into_iter()
        .map(|(l, _)| l)
        .filter(|l| seen.insert(*l))
        .collect();
    join(&lines, dominant_break(text), ends_with_break(text))
}

/// Remove spaces and tabs at the end of every line; breaks are unchanged.
pub fn trim_trailing_whitespace(text: &str) -> String {
    lines_with_breaks(text)
        .into_iter()
        .map(|(l, b)| format!("{}{b}", l.trim_end_matches([' ', '\t'])))
        .collect()
}

/// Convert every line break (LF, CRLF, lone CR) to `to`.
pub fn convert_newlines(text: &str, to: Newline) -> String {
    let brk = match to {
        Newline::Lf => "\n",
        Newline::CrLf => "\r\n",
    };
    lines_with_breaks(text)
        .into_iter()
        .map(|(l, b)| {
            if b.is_empty() {
                l.to_owned()
            } else {
                format!("{l}{brk}")
            }
        })
        .collect()
}

pub fn upper_case(text: &str) -> String {
    text.to_uppercase()
}

pub fn lower_case(text: &str) -> String {
    text.to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_and_unique_keep_line_ending_style_and_final_break() {
        assert_eq!(sort_lines("b\r\na\r\nc\r\n", false), "a\r\nb\r\nc\r\n");
        assert_eq!(sort_lines("b\na\nc", true), "c\nb\na");
        assert_eq!(unique_lines("x\ny\nx\nz\ny"), "x\ny\nz");
        assert_eq!(sort_lines("", false), "");
        assert_eq!(sort_lines("Ä\na\nB", false), "B\na\nÄ", "code point order");
    }

    #[test]
    fn trim_and_newline_conversion() {
        assert_eq!(trim_trailing_whitespace("a \t\r\nb  \nc "), "a\r\nb\nc");
        assert_eq!(convert_newlines("a\r\nb\rc\nd", Newline::Lf), "a\nb\nc\nd");
        assert_eq!(convert_newlines("a\nb\n", Newline::CrLf), "a\r\nb\r\n");
    }

    #[test]
    fn case_is_unicode_aware() {
        assert_eq!(upper_case("äöå straße"), "ÄÖÅ STRASSE");
        assert_eq!(lower_case("ÄÖ"), "äö");
    }
}
