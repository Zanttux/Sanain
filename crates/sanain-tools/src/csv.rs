//! RFC 4180 CSV parsing for the grid view, with delimiter detection and
//! minimal, byte-preserving cell edits.
//!
//! Quoted fields may contain delimiters, doubled quotes and line breaks.
//! Parsing never fails outright: an unterminated quote is reported with the
//! line it started on and the rest is shown as one field, so the user still
//! sees the data. Output is capped at [`MAX_ROWS`].
//!
//! Every parsed field also records its exact char span in the source and
//! whether it was quoted, so an edit replaces only that field's characters:
//! other fields, quoting, line endings and anything past the row cap stay as
//! they were. Positions are char indices (what the editor's rope uses).
//! Column insert/delete is not offered: it would rewrite every row.

use std::ops::Range;

pub const MAX_ROWS: usize = 200_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub delimiter: char,
    pub rows: Vec<Vec<String>>,
    /// Where each row of [`Self::rows`] is in the source.
    pub spans: Vec<RowSpan>,
    /// The first line ending in the file (`"\n"` when there is none), used
    /// when a new row has to start after an unterminated last line.
    pub newline: &'static str,
    /// More rows existed than [`MAX_ROWS`].
    pub truncated: bool,
    /// 1-based line of an unterminated quoted field, if any.
    pub unterminated_quote: Option<usize>,
}

/// One field in the source: its char range (quotes included) and whether
/// it started with a quote.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellSpan {
    pub range: Range<usize>,
    pub quoted: bool,
}

/// One record in the source, as char positions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowSpan {
    pub cells: Vec<CellSpan>,
    /// Start of the first field.
    pub start: usize,
    /// End of the last field (where the line ending starts).
    pub end: usize,
    /// After the line ending; equals `end` on a last line without one.
    pub line_end: usize,
    /// The line ending itself: `"\r\n"`, `"\n"`, `"\r"` or `""`.
    pub ending: &'static str,
}

/// A minimal text change: replace the chars in `range` with `text`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edit {
    pub range: Range<usize>,
    pub text: String,
}

pub const CANDIDATES: [char; 4] = [',', ';', '\t', '|'];

/// Guess the delimiter: the candidate that occurs (outside quotes) the same
/// non-zero number of times on the most of the first lines. Ties prefer the
/// order of [`CANDIDATES`]; tab wins for `.tsv` via `hint`.
pub fn detect_delimiter(text: &str, hint: Option<char>) -> char {
    if let Some(h) = hint {
        return h;
    }
    let lines: Vec<&str> = text.lines().take(20).collect();
    let mut best = (',', 0usize);
    for d in CANDIDATES {
        let counts: Vec<usize> = lines.iter().map(|l| count_outside_quotes(l, d)).collect();
        let Some(&first) = counts.first() else { break };
        if first == 0 {
            continue;
        }
        let consistent = counts.iter().filter(|&&c| c == first).count();
        if consistent > best.1 {
            best = (d, consistent);
        }
    }
    best.0
}

fn count_outside_quotes(line: &str, d: char) -> usize {
    let mut quoted = false;
    line.chars()
        .filter(|&c| {
            if c == '"' {
                quoted = !quoted;
            }
            !quoted && c == d
        })
        .count()
}

pub fn parse(text: &str, delimiter: char) -> Table {
    let mut table = Table {
        delimiter,
        rows: Vec::new(),
        spans: Vec::new(),
        newline: "\n",
        truncated: false,
        unterminated_quote: None,
    };
    let mut newline: Option<&'static str> = None;
    let mut row: Vec<String> = Vec::new();
    let mut cells: Vec<CellSpan> = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut quote_line = 0;
    let mut line = 1;
    let body = text.strip_prefix('\u{FEFF}');
    // Char index of the next char from `chars`.
    let mut pos = usize::from(body.is_some());
    let mut chars = body.unwrap_or(text).chars().peekable();
    let mut field_started = false;
    let mut field_quoted = false;
    let mut field_start = pos;
    let mut row_start = pos;
    while let Some(c) = chars.next() {
        let at = pos;
        pos += 1;
        if quoted {
            match c {
                '"' if chars.peek() == Some(&'"') => {
                    chars.next();
                    pos += 1;
                    field.push('"');
                }
                '"' => quoted = false,
                '\n' => {
                    line += 1;
                    field.push(c);
                }
                _ => field.push(c),
            }
            continue;
        }
        match c {
            '"' if !field_started => {
                quoted = true;
                quote_line = line;
                field_started = true;
                field_quoted = true;
            }
            c if c == delimiter => {
                row.push(std::mem::take(&mut field));
                cells.push(CellSpan {
                    range: field_start..at,
                    quoted: field_quoted,
                });
                field_started = false;
                field_quoted = false;
                field_start = pos;
            }
            '\r' | '\n' => {
                let crlf = c == '\r' && chars.peek() == Some(&'\n');
                if crlf {
                    chars.next();
                    pos += 1;
                }
                let ending = match (c, crlf) {
                    (_, true) => "\r\n",
                    ('\r', _) => "\r",
                    _ => "\n",
                };
                newline.get_or_insert(ending);
                line += 1;
                row.push(std::mem::take(&mut field));
                cells.push(CellSpan {
                    range: field_start..at,
                    quoted: field_quoted,
                });
                table.rows.push(std::mem::take(&mut row));
                table.spans.push(RowSpan {
                    cells: std::mem::take(&mut cells),
                    start: row_start,
                    end: at,
                    line_end: pos,
                    ending,
                });
                field_started = false;
                field_quoted = false;
                field_start = pos;
                row_start = pos;
                if table.rows.len() == MAX_ROWS {
                    table.truncated = chars.peek().is_some();
                    table.newline = newline.unwrap_or("\n");
                    return table;
                }
            }
            _ => {
                field.push(c);
                field_started = true;
            }
        }
    }
    if quoted {
        table.unterminated_quote = Some(quote_line);
    }
    if field_started || !field.is_empty() || !row.is_empty() {
        row.push(field);
        cells.push(CellSpan {
            range: field_start..pos,
            quoted: field_quoted,
        });
        table.rows.push(row);
        table.spans.push(RowSpan {
            cells,
            start: row_start,
            end: pos,
            line_end: pos,
            ending: "",
        });
    }
    table.newline = newline.unwrap_or("\n");
    table
}

/// `value` as a field: quoted when it holds the delimiter, a quote or a line
/// break, or when `keep_quotes` (the field it replaces was quoted). Inner
/// quotes are doubled.
pub fn encode_field(value: &str, delimiter: char, keep_quotes: bool) -> String {
    let needs = keep_quotes || value.contains([delimiter, '"', '\r', '\n']);
    if !needs {
        return value.to_owned();
    }
    let mut s = String::with_capacity(value.len() + 2);
    s.push('"');
    s.push_str(&value.replace('"', "\"\""));
    s.push('"');
    s
}

impl Table {
    /// Edits are refused while a quote is unterminated: the spans past it
    /// are a guess.
    fn editable(&self) -> bool {
        self.unterminated_quote.is_none()
    }

    /// Set the field at `row`, `col` (0-based, the header is row 0) to
    /// `value`. Only that field's chars change. A cell past the end of a
    /// short (ragged) row is created by appending delimiters to that row.
    /// `None` when nothing would change or the edit is refused.
    pub fn edit_cell(&self, row: usize, col: usize, value: &str) -> Option<Edit> {
        if !self.editable() {
            return None;
        }
        let span = self.spans.get(row)?;
        let unchanged = match self.rows[row].get(col) {
            Some(old) => old == value,
            None => value.is_empty(),
        };
        if unchanged {
            return None;
        }
        Some(match span.cells.get(col) {
            Some(cell) => {
                // An emptied lone field must stay a row: at the end of a
                // file without a final line ending, or between a lone CR
                // and this row's LF (which would read as one CRLF), write
                // it as `""`.
                let prev_cr = row
                    .checked_sub(1)
                    .is_some_and(|p| self.spans[p].ending == "\r");
                let lone = span.cells.len() == 1
                    && (span.ending.is_empty() || prev_cr && span.ending == "\n");
                Edit {
                    range: cell.range.clone(),
                    text: encode_field(
                        value,
                        self.delimiter,
                        cell.quoted || lone && value.is_empty(),
                    ),
                }
            }
            None => {
                let mut text: String =
                    std::iter::repeat_n(self.delimiter, col + 1 - span.cells.len()).collect();
                text.push_str(&encode_field(value, self.delimiter, false));
                Edit {
                    range: span.end..span.end,
                    text,
                }
            }
        })
    }

    /// Insert an empty row after `row`, with as many fields as the widest
    /// row. It ends with the line ending of `row`, or, after a last line
    /// without one, starts with the file's first line ending so the file
    /// still has no final newline.
    pub fn insert_row_after(&self, row: usize) -> Option<Edit> {
        if !self.editable() {
            return None;
        }
        let span = self.spans.get(row)?;
        let cols = self.rows.iter().map(Vec::len).max().unwrap_or(1).max(1);
        let fields: String = std::iter::repeat_n(self.delimiter, cols - 1).collect();
        Some(if span.ending.is_empty() {
            // A one-column empty row at the very end would read as a final
            // line ending: write it as `""`.
            let fields = if fields.is_empty() {
                "\"\"".to_owned()
            } else {
                fields
            };
            Edit {
                range: span.end..span.end,
                text: format!("{}{fields}", self.newline),
            }
        } else {
            Edit {
                range: span.line_end..span.line_end,
                text: format!("{fields}{}", span.ending),
            }
        })
    }

    /// Remove `row` and its line ending. Removing an unterminated last row
    /// removes the line ending before it instead, so the file keeps having
    /// no final newline (unless the row before is an empty line). `None`
    /// in the one case that would merge two rows: a mixed-ending file
    /// where a lone CR would meet an empty LF line.
    pub fn delete_row(&self, row: usize) -> Option<Edit> {
        if !self.editable() {
            return None;
        }
        let span = self.spans.get(row)?;
        let prev = row.checked_sub(1).and_then(|p| self.spans.get(p));
        let next = self.spans.get(row + 1);
        // A lone CR before and an empty LF line after would join into one
        // CRLF and lose a row (only in mixed-ending files): refuse.
        if prev.is_some_and(|p| p.ending == "\r")
            && next.is_some_and(|n| n.start == n.end && n.ending == "\n")
        {
            return None;
        }
        let range = match prev {
            // An empty line before would become a bare final line ending
            // and vanish: keep its ending, remove only this row's text.
            Some(prev) if span.ending.is_empty() && prev.start == prev.end => span.start..span.end,
            Some(prev) if span.ending.is_empty() => prev.end..span.end,
            _ => span.start..span.line_end,
        };
        Some(Edit {
            range,
            text: String::new(),
        })
    }
}

/// Parse `text` and set one field; see [`Table::edit_cell`].
pub fn edit_cell(text: &str, row: usize, col: usize, value: &str, delimiter: char) -> Option<Edit> {
    parse(text, delimiter).edit_cell(row, col, value)
}

/// Parse `text` and insert an empty row after `row`; see
/// [`Table::insert_row_after`].
pub fn insert_row_after(text: &str, row: usize, delimiter: char) -> Option<Edit> {
    parse(text, delimiter).insert_row_after(row)
}

/// Parse `text` and delete `row`; see [`Table::delete_row`].
pub fn delete_row(text: &str, row: usize, delimiter: char) -> Option<Edit> {
    parse(text, delimiter).delete_row(row)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quoted_fields_newlines_and_escapes() {
        let t = parse(
            "name,note\r\n\"Doe, J\",\"said \"\"hi\"\"\nthen left\"\r\nä,🙂\n",
            ',',
        );
        assert_eq!(
            t.rows,
            vec![
                vec!["name", "note"],
                vec!["Doe, J", "said \"hi\"\nthen left"],
                vec!["ä", "🙂"],
            ]
        );
        assert_eq!(t.unterminated_quote, None);
    }

    #[test]
    fn empty_fields_and_no_final_newline() {
        assert_eq!(
            parse("a,,c\n,,", ',').rows,
            vec![vec!["a", "", "c"], vec!["", "", ""]]
        );
        assert_eq!(parse("", ',').rows, Vec::<Vec<String>>::new());
        assert_eq!(parse("x", ',').rows, vec![vec!["x"]]);
    }

    #[test]
    fn unterminated_quote_is_reported_not_fatal() {
        let t = parse("a,b\n\"open,c\nd\n", ',');
        assert_eq!(t.unterminated_quote, Some(2));
        assert_eq!(t.rows[0], vec!["a", "b"]);
    }

    #[test]
    fn delimiter_detection() {
        assert_eq!(detect_delimiter("a;b;c\n1;2;3\n", None), ';');
        assert_eq!(detect_delimiter("a\tb\n1\t2\n", None), '\t');
        assert_eq!(
            detect_delimiter("\"x,y\";z\n1;2\n", None),
            ';',
            "comma inside quotes ignored"
        );
        assert_eq!(detect_delimiter("plain", None), ',');
        assert_eq!(detect_delimiter("a,b", Some('\t')), '\t');
    }

    #[test]
    fn row_cap() {
        let big = "x\n".repeat(MAX_ROWS + 5);
        let t = parse(&big, ',');
        assert_eq!(t.rows.len(), MAX_ROWS);
        assert!(t.truncated);
    }

    /// Apply `e` (char positions) to `text`.
    fn apply(text: &str, e: &Edit) -> String {
        let b = |c: usize| text.char_indices().nth(c).map_or(text.len(), |(i, _)| i);
        format!(
            "{}{}{}",
            &text[..b(e.range.start)],
            e.text,
            &text[b(e.range.end)..]
        )
    }

    /// Edit one cell, check the result reparses to the new value with every
    /// other cell unchanged, and return the new text.
    fn round_trip(text: &str, d: char, row: usize, col: usize, value: &str) -> String {
        let before = parse(text, d);
        let e = before.edit_cell(row, col, value).expect("an edit");
        let out = apply(text, &e);
        let after = parse(&out, d);
        assert_eq!(after.rows.len(), before.rows.len(), "{out:?}");
        for (r, cells) in after.rows.iter().enumerate() {
            for (c, v) in cells.iter().enumerate() {
                if (r, c) == (row, col) {
                    assert_eq!(v, value, "{out:?}");
                } else {
                    let old = before.rows[r].get(c).map_or("", String::as_str);
                    assert_eq!(v, old, "cell {r},{c} in {out:?}");
                }
            }
        }
        out
    }

    #[test]
    fn spans_cover_exact_source_chars() {
        let text = "a,\"b,c\"\r\nä🙂,\"x\"\"y\"\n,z";
        let t = parse(text, ',');
        let chars: Vec<char> = text.chars().collect();
        let raw = |r: &Range<usize>| chars[r.clone()].iter().collect::<String>();
        let cells: Vec<Vec<(String, bool)>> = t
            .spans
            .iter()
            .map(|s| s.cells.iter().map(|c| (raw(&c.range), c.quoted)).collect())
            .collect();
        assert_eq!(
            cells,
            vec![
                vec![("a".into(), false), ("\"b,c\"".into(), true)],
                vec![("ä🙂".into(), false), ("\"x\"\"y\"".into(), true)],
                vec![("".into(), false), ("z".into(), false)],
            ]
        );
        assert_eq!(t.spans[0].ending, "\r\n");
        assert_eq!(t.spans[1].ending, "\n");
        assert_eq!(t.spans[2].ending, "");
        assert_eq!(t.spans[2].line_end, chars.len());
        assert_eq!(t.newline, "\r\n");
    }

    #[test]
    fn edit_replaces_only_that_field() {
        let text = "id,name,note\r\n1,Ville,\"hi\"\r\n2,Äiti,🙂\r\n";
        assert_eq!(
            round_trip(text, ',', 1, 1, "Matti"),
            "id,name,note\r\n1,Matti,\"hi\"\r\n2,Äiti,🙂\r\n"
        );
        // Multibyte before and after the edited field.
        assert_eq!(
            round_trip(text, ',', 2, 1, "Öljy"),
            "id,name,note\r\n1,Ville,\"hi\"\r\n2,Öljy,🙂\r\n"
        );
        assert_eq!(
            round_trip(text, ',', 2, 2, "😀ok"),
            "id,name,note\r\n1,Ville,\"hi\"\r\n2,Äiti,😀ok\r\n"
        );
    }

    #[test]
    fn quoting_only_when_needed_or_kept() {
        let text = "a,\"b\",c\n";
        assert_eq!(round_trip(text, ',', 0, 0, "x,y"), "\"x,y\",\"b\",c\n");
        assert_eq!(
            round_trip(text, ',', 0, 0, "say \"hi\""),
            "\"say \"\"hi\"\"\",\"b\",c\n"
        );
        assert_eq!(
            round_trip(text, ',', 0, 0, "two\nlines"),
            "\"two\nlines\",\"b\",c\n"
        );
        assert_eq!(
            round_trip(text, ',', 0, 0, "cr\rhere"),
            "\"cr\rhere\",\"b\",c\n"
        );
        assert_eq!(round_trip(text, ',', 0, 0, " pad "), " pad ,\"b\",c\n");
        // A quoted field stays quoted, even when it no longer needs it.
        assert_eq!(round_trip(text, ',', 0, 1, "plain"), "a,\"plain\",c\n");
        assert_eq!(round_trip(text, ',', 0, 1, ""), "a,\"\",c\n");
        // A semicolon only needs quoting with a semicolon delimiter.
        assert_eq!(round_trip(text, ',', 0, 2, "x;y"), "a,\"b\",x;y\n");
        assert_eq!(round_trip("a;b\n", ';', 0, 1, "x;y"), "a;\"x;y\"\n");
        assert_eq!(round_trip("a;b\n", ';', 0, 1, "x,y"), "a;x,y\n");
    }

    #[test]
    fn crlf_and_last_line_without_newline() {
        let text = "a,b\r\nc,d";
        assert_eq!(round_trip(text, ',', 1, 1, "e"), "a,b\r\nc,e");
        assert_eq!(round_trip(text, ',', 0, 1, "f"), "a,f\r\nc,d");
        // Lone CR line endings too.
        assert_eq!(round_trip("a,b\rc,d\r", ',', 1, 0, "z"), "a,b\rz,d\r");
    }

    #[test]
    fn ragged_rows_are_padded_with_delimiters() {
        let text = "a,b,c\r\nd\r\ne,f,g\r\n";
        assert_eq!(
            round_trip(text, ',', 1, 2, "x"),
            "a,b,c\r\nd,,x\r\ne,f,g\r\n"
        );
        assert_eq!(
            round_trip(text, ',', 1, 1, "y,z"),
            "a,b,c\r\nd,\"y,z\"\r\ne,f,g\r\n"
        );
        // Clearing a cell that does not exist changes nothing.
        assert_eq!(parse(text, ',').edit_cell(1, 2, ""), None);
        // Emptying a lone last field keeps the row.
        assert_eq!(round_trip("a\nb", ',', 1, 0, ""), "a\n\"\"");
        // An empty line is a row of one empty field.
        assert_eq!(round_trip("a,b\n\nc,d\n", ',', 1, 1, "q"), "a,b\n,q\nc,d\n");
    }

    #[test]
    fn tsv_and_bom() {
        let text = "\u{FEFF}k\tv\nä\t\"t\tab\"\n";
        assert_eq!(
            round_trip(text, '\t', 0, 0, "key"),
            "\u{FEFF}key\tv\nä\t\"t\tab\"\n"
        );
        assert_eq!(
            round_trip(text, '\t', 1, 1, "x\ty"),
            "\u{FEFF}k\tv\nä\t\"x\ty\"\n"
        );
        assert_eq!(
            round_trip(text, '\t', 1, 0, "a,b"),
            "\u{FEFF}k\tv\na,b\t\"t\tab\"\n"
        );
        assert_eq!(
            edit_cell(text, 1, 0, "ö", '\t').map(|e| apply(text, &e)),
            Some("\u{FEFF}k\tv\nö\t\"t\tab\"\n".into())
        );
    }

    #[test]
    fn multiline_quoted_field_edit_keeps_later_rows() {
        let text = "a,\"one\r\ntwo\"\r\nb,c\r\n";
        assert_eq!(
            round_trip(text, ',', 0, 1, "single"),
            "a,\"single\"\r\nb,c\r\n"
        );
        assert_eq!(
            round_trip(text, ',', 1, 1, "d"),
            "a,\"one\r\ntwo\"\r\nb,d\r\n"
        );
    }

    #[test]
    fn unchanged_out_of_range_and_unterminated_are_refused() {
        let t = parse("a,b\n", ',');
        assert_eq!(t.edit_cell(0, 0, "a"), None);
        assert_eq!(t.edit_cell(5, 0, "x"), None);
        let bad = parse("a,b\n\"open,c\n", ',');
        assert_eq!(bad.edit_cell(0, 0, "x"), None);
        assert_eq!(bad.insert_row_after(0), None);
        assert_eq!(bad.delete_row(0), None);
    }

    #[test]
    fn insert_row_keeps_line_endings() {
        let text = "a,b,c\r\nd,e,f\r\n";
        let t = parse(text, ',');
        assert_eq!(
            apply(text, &t.insert_row_after(0).unwrap()),
            "a,b,c\r\n,,\r\nd,e,f\r\n"
        );
        assert_eq!(
            apply(text, &t.insert_row_after(1).unwrap()),
            "a,b,c\r\nd,e,f\r\n,,\r\n"
        );
        // After a last line without a line ending, the file still has none.
        let text = "a;b\r\nc;d";
        let t = parse(text, ';');
        let out = apply(text, &t.insert_row_after(1).unwrap());
        assert_eq!(out, "a;b\r\nc;d\r\n;");
        assert_eq!(parse(&out, ';').rows[2], vec!["", ""]);
        assert_eq!(
            apply("x", &insert_row_after("x", 0, ',').unwrap()),
            "x\n\"\"",
            "one column at the end: an empty quoted field, LF by default"
        );
        assert_eq!(insert_row_after("", 0, ','), None);
    }

    #[test]
    fn delete_row_removes_exactly_one_record() {
        let text = "h1,h2\r\n\"a\r\nb\",c\r\nd,e\r\n";
        let del = |r| apply(text, &delete_row(text, r, ',').unwrap());
        assert_eq!(del(0), "\"a\r\nb\",c\r\nd,e\r\n");
        assert_eq!(del(1), "h1,h2\r\nd,e\r\n");
        assert_eq!(del(2), "h1,h2\r\n\"a\r\nb\",c\r\n");
        // Unterminated last row: the ending before it goes, not after.
        let text = "a\nb\nc";
        assert_eq!(apply(text, &delete_row(text, 2, ',').unwrap()), "a\nb");
        assert_eq!(apply(text, &delete_row(text, 0, ',').unwrap()), "b\nc");
        assert_eq!(apply("only", &delete_row("only", 0, ',').unwrap()), "");
        assert_eq!(delete_row(text, 3, ','), None);
    }

    #[test]
    fn encode_field_rules() {
        assert_eq!(encode_field("x", ',', false), "x");
        assert_eq!(encode_field("x", ',', true), "\"x\"");
        assert_eq!(encode_field("", ',', false), "");
        assert_eq!(encode_field("\"", ',', false), "\"\"\"\"");
        assert_eq!(encode_field("a|b", '|', false), "\"a|b\"");
    }

    proptest::proptest! {
        /// Any edit of any cell in any well-formed input reparses to the new
        /// value with every other cell unchanged, and leaves the text before
        /// and after the edited range untouched.
        #[test]
        fn edit_round_trips(
            text in "[a\u{e4}\u{1F642},;\" \r\n]{0,30}",
            value in "[b\u{f6},\"\r\n ]{0,6}",
            row in 0usize..6,
            col in 0usize..4,
        ) {
            let t = parse(&text, ',');
            if let Some(e) = t.edit_cell(row, col, &value) {
                let out = apply(&text, &e);
                let n = text.chars().count();
                let m = out.chars().count();
                let prefix: String = text.chars().take(e.range.start).collect();
                let suffix: String = text.chars().skip(e.range.end).collect();
                proptest::prop_assert!(out.starts_with(&prefix));
                proptest::prop_assert!(out.ends_with(&suffix));
                proptest::prop_assert_eq!(m, n - e.range.len() + e.text.chars().count());
                let after = parse(&out, ',');
                proptest::prop_assert_eq!(after.rows.len(), t.rows.len());
                for (r, cells) in after.rows.iter().enumerate() {
                    for (c, v) in cells.iter().enumerate() {
                        let want = if (r, c) == (row, col) {
                            value.as_str()
                        } else {
                            t.rows[r].get(c).map_or("", String::as_str)
                        };
                        proptest::prop_assert_eq!(v, want);
                    }
                }
            }
        }

        /// Inserting or deleting a row changes exactly that row.
        #[test]
        fn row_ops_round_trip(
            text in "[a\u{e4},\" \r\n]{0,30}",
            row in 0usize..6,
        ) {
            let t = parse(&text, ',');
            if let Some(e) = t.insert_row_after(row) {
                let after = parse(&apply(&text, &e), ',');
                let mut want = t.rows.clone();
                let cols = t.rows.iter().map(Vec::len).max().unwrap_or(1).max(1);
                want.insert(row + 1, vec![String::new(); cols]);
                proptest::prop_assert_eq!(after.rows, want);
            }
            if let Some(e) = t.delete_row(row) {
                let after = parse(&apply(&text, &e), ',');
                let mut want = t.rows.clone();
                want.remove(row);
                proptest::prop_assert_eq!(after.rows, want);
            }
        }
    }
}
