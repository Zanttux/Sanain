//! RFC 4180 CSV parsing for a read-only grid, with delimiter detection.
//!
//! Quoted fields may contain delimiters, doubled quotes and line breaks.
//! Parsing never fails outright: an unterminated quote is reported with the
//! line it started on and the rest is shown as one field, so the user still
//! sees the data. Output is capped at [`MAX_ROWS`].

pub const MAX_ROWS: usize = 200_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub delimiter: char,
    pub rows: Vec<Vec<String>>,
    /// More rows existed than [`MAX_ROWS`].
    pub truncated: bool,
    /// 1-based line of an unterminated quoted field, if any.
    pub unterminated_quote: Option<usize>,
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
        truncated: false,
        unterminated_quote: None,
    };
    let mut row: Vec<String> = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut quote_line = 0;
    let mut line = 1;
    let mut chars = text
        .strip_prefix('\u{FEFF}')
        .unwrap_or(text)
        .chars()
        .peekable();
    let mut field_started = false;
    while let Some(c) = chars.next() {
        if quoted {
            match c {
                '"' if chars.peek() == Some(&'"') => {
                    chars.next();
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
            }
            c if c == delimiter => {
                row.push(std::mem::take(&mut field));
                field_started = false;
            }
            '\r' if chars.peek() == Some(&'\n') => {}
            '\n' | '\r' => {
                line += 1;
                row.push(std::mem::take(&mut field));
                field_started = false;
                table.rows.push(std::mem::take(&mut row));
                if table.rows.len() == MAX_ROWS {
                    table.truncated = chars.peek().is_some();
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
        table.rows.push(row);
    }
    table
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
}
