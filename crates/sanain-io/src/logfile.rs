//! Viewing very large or growing files (logs) line by line without loading
//! them into memory.
//!
//! A [`LineIndex`] records where each line starts. It is built in chunks so
//! a background thread can index while the UI shows what is known so far,
//! and extended when the file grows. Lines are read from disk on demand and
//! decoded for display only (lossy UTF-8); this module never writes.

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom};
use std::ops::Range;
use std::sync::atomic::{AtomicBool, Ordering};

/// Bytes read per indexing step.
pub const CHUNK: usize = 4 * 1024 * 1024;

/// Byte offsets of line starts, for the bytes `0..end` of a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineIndex {
    starts: Vec<u64>,
    end: u64,
}

impl Default for LineIndex {
    fn default() -> Self {
        Self {
            starts: vec![0],
            end: 0,
        }
    }
}

/// Line starts (offset after each `\n`) inside `chunk`, which begins at
/// byte `base` of the file.
pub fn starts_in(chunk: &[u8], base: u64) -> Vec<u64> {
    chunk
        .iter()
        .enumerate()
        .filter(|(_, b)| **b == b'\n')
        .map(|(i, _)| base + i as u64 + 1)
        .collect()
}

impl LineIndex {
    /// Bytes indexed so far.
    pub fn end(&self) -> u64 {
        self.end
    }

    /// Number of lines in `0..end`. A final line break does not start an
    /// extra empty line.
    pub fn line_count(&self) -> usize {
        match self.starts.last() {
            _ if self.end == 0 => 0,
            Some(&last) if last == self.end => self.starts.len() - 1,
            _ => self.starts.len(),
        }
    }

    /// Byte range of `line`, including its line break.
    pub fn line_bytes(&self, line: usize) -> Option<Range<u64>> {
        if line >= self.line_count() {
            return None;
        }
        let end = self.starts.get(line + 1).copied().unwrap_or(self.end);
        Some(self.starts[line]..end)
    }

    /// The line starting exactly at byte `offset`.
    pub fn line_at(&self, offset: u64) -> Option<usize> {
        self.starts.binary_search(&offset).ok()
    }

    /// Append starts found by [`starts_in`] for bytes up to `end`.
    pub fn append(&mut self, starts: &[u64], end: u64) {
        debug_assert!(end >= self.end);
        self.starts.extend_from_slice(starts);
        self.end = end;
    }

    /// Index `reader` (positioned at [`Self::end`]) until EOF. Returns
    /// `false` if `cancel` was set first.
    pub fn scan(&mut self, reader: &mut impl Read, cancel: &AtomicBool) -> io::Result<bool> {
        let mut buf = vec![0; CHUNK];
        loop {
            if cancel.load(Ordering::Relaxed) {
                return Ok(false);
            }
            let n = match reader.read(&mut buf) {
                Ok(0) => return Ok(true),
                Ok(n) => n,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };
            let starts = starts_in(&buf[..n], self.end);
            let end = self.end + n as u64;
            self.append(&starts, end);
        }
    }
}

/// Text of one line for display: at most `max_bytes` bytes, line break
/// removed, invalid UTF-8 replaced. The flag tells whether it was cut.
pub fn display_line(bytes: &[u8], max_bytes: usize) -> (String, bool) {
    let cut = bytes.len() > max_bytes;
    let mut b = &bytes[..bytes.len().min(max_bytes)];
    if !cut {
        b = b.strip_suffix(b"\n").unwrap_or(b);
        b = b.strip_suffix(b"\r").unwrap_or(b);
    }
    (String::from_utf8_lossy(b).into_owned(), cut)
}

/// Read `lines` of `file` for display (see [`display_line`]).
pub fn read_lines(
    file: &mut File,
    index: &LineIndex,
    lines: Range<usize>,
    max_bytes: usize,
) -> io::Result<Vec<(String, bool)>> {
    let mut out = Vec::with_capacity(lines.len());
    let mut buf = Vec::new();
    for line in lines {
        let Some(r) = index.line_bytes(line) else {
            break;
        };
        let want = (r.end - r.start).min(max_bytes as u64 + 1);
        file.seek(SeekFrom::Start(r.start))?;
        buf.clear();
        file.by_ref().take(want).read_to_end(&mut buf)?;
        out.push(display_line(&buf, max_bytes));
    }
    Ok(out)
}

/// Lines in the first `len` bytes of `reader` (whose first line is number
/// `first_line`) for which `keep` is true, reported in batches through
/// `found(lines, bytes_done)`. Lines longer than `max_bytes` are tested on
/// their first `max_bytes`. A final line without a line break is tested
/// only if `include_partial`, and is never counted in `bytes_done`, so a
/// later pass over a grown file starts at that line again. Returns `false`
/// if cancelled.
#[allow(clippy::too_many_arguments)]
pub fn filter_lines(
    reader: impl Read,
    first_line: usize,
    len: u64,
    max_bytes: usize,
    include_partial: bool,
    cancel: &AtomicBool,
    mut keep: impl FnMut(&str) -> bool,
    mut found: impl FnMut(Vec<usize>, u64),
) -> io::Result<bool> {
    let mut r = BufReader::with_capacity(1 << 20, reader.take(len));
    let mut line = first_line;
    let mut pos = 0u64;
    let mut buf = Vec::new();
    let mut batch = Vec::new();
    loop {
        if line.is_multiple_of(4096) {
            if cancel.load(Ordering::Relaxed) {
                return Ok(false);
            }
            if !batch.is_empty() {
                found(std::mem::take(&mut batch), pos);
            }
        }
        buf.clear();
        let n = read_line_capped(&mut r, &mut buf, max_bytes)?;
        if n == 0 {
            break;
        }
        pos += n as u64;
        // A final partial line (no line break yet) is left for the next
        // pass, so a growing file is not matched on half a line.
        if !buf.ends_with(b"\n") && n <= max_bytes {
            pos -= n as u64;
            if include_partial && keep(&display_line(&buf, max_bytes).0) {
                batch.push(line);
            }
            break;
        }
        if keep(&display_line(&buf, max_bytes).0) {
            batch.push(line);
        }
        line += 1;
    }
    found(batch, pos);
    Ok(true)
}

/// Read one line into `buf`, keeping at most `max` bytes (+ the break when
/// it fits) but consuming the whole line. Returns bytes consumed.
fn read_line_capped(r: &mut impl BufRead, buf: &mut Vec<u8>, max: usize) -> io::Result<usize> {
    let mut consumed = 0;
    loop {
        let avail = match r.fill_buf() {
            Ok(a) => a,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        if avail.is_empty() {
            return Ok(consumed);
        }
        let (take, done) = match avail.iter().position(|b| *b == b'\n') {
            Some(i) => (i + 1, true),
            None => (avail.len(), false),
        };
        let room = (max + 1).saturating_sub(buf.len());
        buf.extend_from_slice(&avail[..take.min(room)]);
        r.consume(take);
        consumed += take;
        if done {
            return Ok(consumed);
        }
    }
}

/// Severity of a log line, for colouring.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
}

/// The level named near the start of a log line (first 8 words, case
/// insensitive): `ERROR`, `[warn]`, `level=debug`, `E/`... Words later in
/// the message do not count, so "failed with error" in a message body is
/// not mistaken for the level.
pub fn level(line: &str) -> Option<Level> {
    let head: String = line.chars().take(200).collect();
    head.split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| !w.is_empty())
        .take(8)
        .find_map(|w| match w.to_ascii_uppercase().as_str() {
            "ERROR" | "ERR" | "FATAL" | "CRITICAL" | "CRIT" | "PANIC" | "SEVERE" | "EMERG"
            | "ALERT" => Some(Level::Error),
            "WARN" | "WARNING" => Some(Level::Warn),
            "INFO" | "NOTICE" => Some(Level::Info),
            "DEBUG" | "TRACE" | "VERBOSE" | "FINE" => Some(Level::Debug),
            _ => None,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn index(text: &[u8]) -> LineIndex {
        let mut i = LineIndex::default();
        assert!(
            i.scan(&mut Cursor::new(text), &AtomicBool::new(false))
                .unwrap()
        );
        i
    }

    #[test]
    fn detects_levels_near_the_start() {
        assert_eq!(
            level("2026-11-25T10:00:00Z ERROR worker failed"),
            Some(Level::Error)
        );
        assert_eq!(level("[warn] disk almost full"), Some(Level::Warn));
        assert_eq!(level("ts=1 level=debug msg=hi"), Some(Level::Debug));
        assert_eq!(
            level("Nov 25 10:00 host app[12]: INFO started"),
            Some(Level::Info)
        );
        assert_eq!(level("plain line"), None);
        assert_eq!(
            level("2026-11-25 10:00:00 request 1 2 3 4 5 handled after an error"),
            None,
            "words late in the message are not the level"
        );
    }

    #[test]
    fn counts_lines_like_a_viewer() {
        assert_eq!(index(b"").line_count(), 0);
        assert_eq!(index(b"a").line_count(), 1);
        assert_eq!(index(b"a\n").line_count(), 1);
        assert_eq!(index(b"a\nb").line_count(), 2);
        assert_eq!(index(b"a\n\n").line_count(), 2);
        let i = index(b"ab\r\ncd\nef");
        assert_eq!(i.line_bytes(0), Some(0..4));
        assert_eq!(i.line_bytes(1), Some(4..7));
        assert_eq!(i.line_bytes(2), Some(7..9));
        assert_eq!(i.line_bytes(3), None);
    }

    #[test]
    fn chunked_and_incremental_indexing_agree() {
        let text: Vec<u8> = (0..5000)
            .flat_map(|n| format!("line {n}\n").into_bytes())
            .collect();
        let whole = index(&text);
        let mut parts = LineIndex::default();
        for chunk in text.chunks(777) {
            let s = starts_in(chunk, parts.end());
            let end = parts.end() + chunk.len() as u64;
            parts.append(&s, end);
        }
        assert_eq!(parts, whole);
        assert_eq!(whole.line_count(), 5000);
    }

    #[test]
    fn reads_display_lines_from_disk() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("x.log");
        std::fs::write(&p, b"one\r\ntw\xffo\nthree-long-line\nlast").unwrap();
        let i = index(&std::fs::read(&p).unwrap());
        let mut f = File::open(&p).unwrap();
        let lines = read_lines(&mut f, &i, 0..10, 5).unwrap();
        assert_eq!(
            lines,
            vec![
                ("one".into(), false),
                ("tw\u{fffd}o".into(), false),
                ("three".into(), true),
                ("last".into(), false),
            ]
        );
    }

    #[test]
    fn filters_complete_lines_and_cancels() {
        let text = b"err 1\nok\nerr 2\nok\nerr partial";
        let mut got = Vec::new();
        let mut done_at = 0;
        let no = AtomicBool::new(false);
        let ok = filter_lines(
            Cursor::new(&text[..]),
            0,
            text.len() as u64,
            100,
            false,
            &no,
            |l| l.contains("err"),
            |b, pos| {
                got.extend(b);
                done_at = pos;
            },
        )
        .unwrap();
        assert!(ok);
        assert_eq!(got, vec![0, 2]);
        assert_eq!(done_at, 18, "partial last line left for later");
        let yes = AtomicBool::new(true);
        assert!(
            !filter_lines(
                Cursor::new(&text[..]),
                0,
                99,
                100,
                false,
                &yes,
                |_| true,
                |_, _| {}
            )
            .unwrap()
        );
        let mut got = Vec::new();
        filter_lines(
            Cursor::new(&text[..]),
            0,
            99,
            100,
            true,
            &no,
            |l| l.contains("err"),
            |b, pos| {
                got.extend(b);
                assert_eq!(pos, 18);
            },
        )
        .unwrap();
        assert_eq!(got, vec![0, 2, 4], "partial line tested on request");
    }

    #[test]
    fn huge_line_is_capped_but_consumed() {
        let mut text = vec![b'x'; 10_000];
        text.extend_from_slice(b"\nneedle\n");
        let mut got = Vec::new();
        filter_lines(
            Cursor::new(&text),
            0,
            text.len() as u64,
            64,
            false,
            &AtomicBool::new(false),
            |l| l.contains("needle") || l.len() > 64,
            |b, _| got.extend(b),
        )
        .unwrap();
        assert_eq!(got, vec![1]);
    }
}
