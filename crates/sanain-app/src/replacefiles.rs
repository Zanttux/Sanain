//! Replace across files found by a folder search (files not open in a tab).
//!
//! Each file is reloaded, so the result reflects what is on disk now; the
//! old bytes go to local history first; the save is refused if the file
//! changed between reading and writing. Encoding, BOM and line endings are
//! kept: only the matched text changes.

use std::path::Path;

use sanain_core::Document;
use sanain_core::search::Matcher;
use sanain_io::history::{HistoryStore, Retention};
use sanain_io::{SaveMode, TextEncoding, load_with_bytes, save};

/// Replace every match of `matcher` in the file at `path` and save it.
/// Returns the number of replacements (0 leaves the file untouched).
pub fn replace_in_file(
    path: &Path,
    matcher: &Matcher,
    with: &str,
    history: Option<&HistoryStore>,
) -> Result<usize, String> {
    let (loaded, bytes) = load_with_bytes(path).map_err(|e| e.to_string())?;
    // The folder search reads files as UTF-8; anything decoded otherwise
    // (e.g. UTF-16 detected later in the file) is left alone.
    if !matches!(loaded.encoding, TextEncoding::Utf8 | TextEncoding::Utf8Bom) {
        return Err("not a UTF-8 file".into());
    }
    let mut doc = Document::from_saved_text(&loaded.text);
    let (tx, n) = matcher.replace_all(doc.rope(), with);
    if n == 0 {
        return Ok(0);
    }
    if let Some(store) = history {
        store
            .record(path, &bytes, Retention::default())
            .map_err(|e| e.to_string())?;
    }
    doc.apply(&tx).map_err(|e| e.to_string())?;
    save(
        path,
        &doc.text(),
        loaded.encoding,
        SaveMode::Expect(&loaded.fingerprint),
    )
    .map_err(|e| e.to_string())?;
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sanain_core::search::Query;

    fn matcher(pattern: &str, regex: bool) -> Matcher {
        Query {
            pattern: pattern.into(),
            regex,
            ..Default::default()
        }
        .compile()
        .unwrap()
        .unwrap()
    }

    #[test]
    fn replaces_and_keeps_bytes_otherwise() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("a.txt");
        std::fs::write(&p, "\u{feff}foo = 1\r\nbar foo\r\n").unwrap();
        let hist = HistoryStore::open(d.path().join("hist")).unwrap();
        let n = replace_in_file(&p, &matcher("foo", false), "baz", Some(&hist)).unwrap();
        assert_eq!(n, 2);
        assert_eq!(
            std::fs::read(&p).unwrap(),
            "\u{feff}baz = 1\r\nbar baz\r\n".as_bytes(),
            "BOM and CRLF kept"
        );
        let versions = hist.list(&p).unwrap();
        assert_eq!(versions.len(), 1, "old contents in history");
        assert_eq!(
            versions[0].read().unwrap(),
            "\u{feff}foo = 1\r\nbar foo\r\n".as_bytes()
        );
        // Regex with captures; no match leaves the file alone.
        let n = replace_in_file(&p, &matcher(r"(\w+) = (\d)", true), "$2 = $1", None).unwrap();
        assert_eq!(n, 1);
        assert!(std::fs::read_to_string(&p).unwrap().contains("1 = baz"));
        let before = std::fs::metadata(&p).unwrap().modified().unwrap();
        assert_eq!(
            replace_in_file(&p, &matcher("nothing", false), "x", None).unwrap(),
            0
        );
        assert_eq!(std::fs::metadata(&p).unwrap().modified().unwrap(), before);
    }
}
