//! JSON validate / format / minify that preserves the source's lexemes.
//!
//! Unlike a parse-then-serialise round trip, tokens are re-emitted exactly
//! as written: duplicate keys stay, numbers keep their spelling (`1.0`,
//! `1e3`, huge integers), and strings keep their escapes. Only whitespace
//! between tokens changes. The parser uses an explicit stack, so nesting
//! depth cannot overflow the call stack.

use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{message} at line {line}, column {column}")]
pub struct JsonError {
    pub message: &'static str,
    /// 1-based line and column (in chars) of the problem.
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tok {
    OpenObj,
    CloseObj,
    OpenArr,
    CloseArr,
    Colon,
    Comma,
    /// String, number, true, false or null.
    Scalar,
}

/// Tokens with their byte ranges, validated against the JSON grammar.
fn tokenize(src: &str) -> Result<Vec<(Tok, Range<usize>)>, JsonError> {
    let b = src.as_bytes();
    let err = |message, at: usize| {
        let before = &src[..at.min(src.len())];
        let line = before.matches('\n').count() + 1;
        let column = before.rsplit('\n').next().map_or(0, |l| l.chars().count()) + 1;
        JsonError {
            message,
            line,
            column,
            offset: at,
        }
    };
    let mut toks = Vec::new();
    let mut i = 0;
    // Skip a UTF-8 BOM if the text still has one.
    if src.starts_with('\u{FEFF}') {
        i = 3;
    }
    while i < b.len() {
        let c = b[i];
        let start = i;
        let tok = match c {
            b' ' | b'\t' | b'\n' | b'\r' => {
                i += 1;
                continue;
            }
            b'{' => Tok::OpenObj,
            b'}' => Tok::CloseObj,
            b'[' => Tok::OpenArr,
            b']' => Tok::CloseArr,
            b':' => Tok::Colon,
            b',' => Tok::Comma,
            b'"' => {
                i += 1;
                loop {
                    match b.get(i) {
                        None => return Err(err("unterminated string", start)),
                        Some(b'"') => break,
                        Some(b'\\') => match b.get(i + 1) {
                            Some(b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't') => i += 2,
                            Some(b'u') => {
                                let hex = b
                                    .get(i + 2..i + 6)
                                    .ok_or_else(|| err("invalid \\u escape", i))?;
                                if !hex.iter().all(u8::is_ascii_hexdigit) {
                                    return Err(err("invalid \\u escape", i));
                                }
                                i += 6;
                            }
                            _ => return Err(err("invalid escape", i)),
                        },
                        Some(&ch) if ch < 0x20 => {
                            return Err(err("control character in string", i));
                        }
                        Some(_) => i += 1,
                    }
                }
                i += 1;
                toks.push((Tok::Scalar, start..i));
                continue;
            }
            b'-' | b'0'..=b'9' => {
                if b[i] == b'-' {
                    i += 1;
                }
                match b.get(i) {
                    Some(b'0') => i += 1,
                    Some(b'1'..=b'9') => {
                        while matches!(b.get(i), Some(b'0'..=b'9')) {
                            i += 1;
                        }
                    }
                    _ => return Err(err("invalid number", start)),
                }
                if b.get(i) == Some(&b'.') {
                    i += 1;
                    if !matches!(b.get(i), Some(b'0'..=b'9')) {
                        return Err(err("invalid number", start));
                    }
                    while matches!(b.get(i), Some(b'0'..=b'9')) {
                        i += 1;
                    }
                }
                if matches!(b.get(i), Some(b'e' | b'E')) {
                    i += 1;
                    if matches!(b.get(i), Some(b'+' | b'-')) {
                        i += 1;
                    }
                    if !matches!(b.get(i), Some(b'0'..=b'9')) {
                        return Err(err("invalid number", start));
                    }
                    while matches!(b.get(i), Some(b'0'..=b'9')) {
                        i += 1;
                    }
                }
                toks.push((Tok::Scalar, start..i));
                continue;
            }
            _ => {
                let rest = &src[i..];
                let word = ["true", "false", "null"]
                    .into_iter()
                    .find(|w| rest.starts_with(w));
                match word {
                    Some(w) => {
                        i += w.len();
                        toks.push((Tok::Scalar, start..i));
                        continue;
                    }
                    None => return Err(err("unexpected character", i)),
                }
            }
        };
        i += 1;
        toks.push((tok, start..i));
    }
    validate(&toks, src, &err)?;
    Ok(toks)
}

#[derive(Clone, Copy, PartialEq)]
enum Ctx {
    Object,
    Array,
}

/// Grammar check with an explicit stack.
fn validate(
    toks: &[(Tok, Range<usize>)],
    src: &str,
    err: &dyn Fn(&'static str, usize) -> JsonError,
) -> Result<(), JsonError> {
    #[derive(Clone, Copy, PartialEq)]
    enum Want {
        Value,
        /// After a value: comma, close, or end.
        After,
        /// In an object: key or `}` (just opened) / key (after comma).
        Key {
            allow_close: bool,
        },
        Colon,
        /// In an array right after `[`: value or `]`.
        ValueOrClose,
    }
    let mut stack: Vec<Ctx> = Vec::new();
    let mut want = Want::Value;
    let mut done = false;
    for (t, r) in toks {
        let at = r.start;
        if done {
            return Err(err("unexpected content after the value", at));
        }
        let is_string = *t == Tok::Scalar && src.as_bytes()[r.start] == b'"';
        want = match (want, t) {
            (Want::Value | Want::ValueOrClose, Tok::Scalar) => Want::After,
            (Want::Value | Want::ValueOrClose, Tok::OpenObj) => {
                stack.push(Ctx::Object);
                Want::Key { allow_close: true }
            }
            (Want::Value | Want::ValueOrClose, Tok::OpenArr) => {
                stack.push(Ctx::Array);
                Want::ValueOrClose
            }
            (Want::ValueOrClose, Tok::CloseArr) => {
                stack.pop();
                Want::After
            }
            (Want::Key { .. }, Tok::Scalar) if is_string => Want::Colon,
            (Want::Key { allow_close: true }, Tok::CloseObj) => {
                stack.pop();
                Want::After
            }
            (Want::Colon, Tok::Colon) => Want::Value,
            (Want::After, Tok::Comma) => match stack.last() {
                Some(Ctx::Object) => Want::Key { allow_close: false },
                Some(Ctx::Array) => Want::Value,
                None => return Err(err("unexpected comma", at)),
            },
            (Want::After, Tok::CloseObj) if stack.last() == Some(&Ctx::Object) => {
                stack.pop();
                Want::After
            }
            (Want::After, Tok::CloseArr) if stack.last() == Some(&Ctx::Array) => {
                stack.pop();
                Want::After
            }
            (Want::Key { .. }, _) => return Err(err("expected a string key", at)),
            (Want::Colon, _) => return Err(err("expected ':'", at)),
            (Want::After, _) => return Err(err("expected ',' or a closing bracket", at)),
            _ => return Err(err("expected a value", at)),
        };
        if want == Want::After && stack.is_empty() {
            done = true;
        }
    }
    if !done {
        let at = src.len();
        return Err(if toks.is_empty() {
            err("empty document", at)
        } else {
            err("unexpected end of input", at)
        });
    }
    Ok(())
}

/// Check that `src` is valid JSON.
pub fn validate_json(src: &str) -> Result<(), JsonError> {
    tokenize(src).map(|_| ())
}

/// Pretty-print with `indent` per level and `newline` between lines.
/// Empty objects and arrays stay on one line (`{}`, `[]`).
pub fn format(src: &str, indent: &str, newline: &str) -> Result<String, JsonError> {
    let toks = tokenize(src)?;
    // Indentation grows with depth, so pathological nesting could expand a
    // small file into gigabytes. Refuse beyond a generous bound.
    let limit = src.len().saturating_mul(16).saturating_add(16 << 20);
    let mut out = String::with_capacity(src.len() + src.len() / 4);
    let mut level = 0usize;
    let line = |out: &mut String, level: usize| {
        out.push_str(newline);
        for _ in 0..level {
            out.push_str(indent);
        }
    };
    for (k, (t, r)) in toks.iter().enumerate() {
        let text = &src[r.clone()];
        let next = toks.get(k + 1).map(|(t, _)| *t);
        match t {
            Tok::OpenObj | Tok::OpenArr => {
                out.push_str(text);
                let empty = matches!(
                    (t, next),
                    (Tok::OpenObj, Some(Tok::CloseObj)) | (Tok::OpenArr, Some(Tok::CloseArr))
                );
                if !empty {
                    level += 1;
                    line(&mut out, level);
                }
            }
            Tok::CloseObj | Tok::CloseArr => {
                let prev = k.checked_sub(1).map(|p| toks[p].0);
                let empty = matches!(prev, Some(Tok::OpenObj | Tok::OpenArr));
                if !empty {
                    level = level.saturating_sub(1);
                    line(&mut out, level);
                }
                out.push_str(text);
            }
            Tok::Comma => {
                out.push(',');
                line(&mut out, level);
            }
            Tok::Colon => out.push_str(": "),
            Tok::Scalar => out.push_str(text),
        }
        if out.len() > limit {
            return Err(JsonError {
                message: "too deeply nested to format",
                line: 1,
                column: 1,
                offset: r.start,
            });
        }
    }
    if src.ends_with('\n') {
        out.push_str(newline);
    }
    Ok(out)
}

/// Remove all whitespace between tokens.
pub fn minify(src: &str) -> Result<String, JsonError> {
    let toks = tokenize(src)?;
    let mut out = String::with_capacity(src.len());
    for (_, r) in toks {
        out.push_str(&src[r]);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_preserves_lexemes_and_duplicates() {
        let src = r#"{"a":1.0,"a":1e3,"big":123456789012345678901234567890,"s":"\u00e4\n","e":{},"l":[]}"#;
        let out = format(src, "  ", "\n").unwrap();
        assert_eq!(
            out,
            "{\n  \"a\": 1.0,\n  \"a\": 1e3,\n  \"big\": 123456789012345678901234567890,\n  \"s\": \"\\u00e4\\n\",\n  \"e\": {},\n  \"l\": []\n}"
        );
        assert_eq!(minify(&out).unwrap(), src);
    }

    #[test]
    fn crlf_tabs_and_trailing_newline() {
        let out = format("[1,[2]]\n", "\t", "\r\n").unwrap();
        assert_eq!(out, "[\r\n\t1,\r\n\t[\r\n\t\t2\r\n\t]\r\n]\r\n");
    }

    #[test]
    fn errors_have_positions() {
        let e = validate_json("{\n  \"a\": 1,\n  \"b\": }").unwrap_err();
        assert_eq!((e.message, e.line, e.column), ("expected a value", 3, 8));
        for bad in [
            "",
            "{",
            "[1,]",
            "{\"a\":1,}",
            "{a:1}",
            "01",
            "1.",
            "-",
            "1e",
            "\"\\x\"",
            "\"a\nb\"",
            "[1] 2",
            "tru",
            "{\"a\" 1}",
            "]",
            "\"unterminated",
            "\"\\u12G4\"",
            "NaN",
        ] {
            assert!(validate_json(bad).is_err(), "{bad:?} should fail");
        }
        for good in [
            "0",
            "-0.5e-7",
            "\"\"",
            "[]",
            "{}",
            "true",
            " null ",
            "\u{FEFF}{}",
            "[[[]]]",
        ] {
            assert!(validate_json(good).is_ok(), "{good:?} should pass");
        }
    }

    #[test]
    fn deep_nesting_has_no_recursion_limit() {
        let deep = "[".repeat(200_000) + &"]".repeat(200_000);
        assert!(validate_json(&deep).is_ok());
        assert!(minify(&deep).is_ok());
        // Output would be ~depth^2 bytes: refused, quickly.
        let t = std::time::Instant::now();
        assert_eq!(
            format(&deep, "  ", "\n").unwrap_err().message,
            "too deeply nested to format"
        );
        assert!(t.elapsed() < std::time::Duration::from_secs(5));
        let moderate = "[".repeat(500) + &"]".repeat(500);
        assert!(format(&moderate, "  ", "\n").is_ok());
        assert!(validate_json(&("[".repeat(100_000))).is_err());
    }

    proptest::proptest! {
        #[test]
        fn minify_of_format_is_minify(s in "[\\[\\]{}:, \"a-c0-9.e\\-\\n]{0,40}") {
            if let Ok(f) = format(&s, "  ", "\n") {
                proptest::prop_assert_eq!(minify(&f).unwrap(), minify(&s).unwrap());
            }
        }
    }
}
