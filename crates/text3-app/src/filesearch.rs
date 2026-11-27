//! Search the text files under a folder, on a background thread.
//!
//! Bounded and cancellable: hidden entries and common build folders are
//! skipped, directory symlinks are not followed (no loops), files over
//! [`MAX_FILE_BYTES`] or that look binary or are not UTF-8 are skipped, and
//! results stop at [`MAX_HITS`]. Nothing is modified.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};

use text3_core::search::Matcher;

pub const MAX_FILE_BYTES: u64 = 10 * 1024 * 1024;
pub const MAX_HITS: usize = 10_000;
const MAX_FILES: usize = 100_000;
const MAX_LINE_CHARS: usize = 300;
const SKIP_DIRS: &[&str] = &[
    "node_modules",
    "target",
    "build",
    "dist",
    "__pycache__",
    "venv",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hit {
    pub path: PathBuf,
    /// 0-based line number.
    pub line: usize,
    /// The line's text, shortened to a window around the match.
    pub preview: String,
    /// Match position in `preview`, as char indices.
    pub preview_match: std::ops::Range<usize>,
    /// Match position in the file, as char indices (for selecting it).
    pub chars: std::ops::Range<usize>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Stats {
    pub files_searched: usize,
    pub files_skipped: usize,
    pub hits: usize,
    /// Stopped at a limit (hits or files) or by cancellation.
    pub truncated: bool,
}

#[derive(Debug)]
pub enum Event {
    Hits(Vec<Hit>),
    Done(Stats),
}

/// A running search. Dropping it cancels the search.
#[derive(Debug)]
pub struct Search {
    cancel: Arc<AtomicBool>,
    pub events: Receiver<Event>,
}

impl Drop for Search {
    fn drop(&mut self) {
        self.cancel.store(true, Ordering::Relaxed);
    }
}

impl Search {
    pub fn start(root: PathBuf, matcher: Matcher, wake: impl Fn() + Send + 'static) -> Self {
        let cancel = Arc::new(AtomicBool::new(false));
        let (tx, events) = channel();
        let flag = cancel.clone();
        let spawned = std::thread::Builder::new()
            .name("sanain-folder-search".into())
            .spawn(move || {
                let stats = run(&root, &matcher, &flag, &tx, &wake);
                let _ = tx.send(Event::Done(stats));
                wake();
            });
        if spawned.is_err() {
            // Report an empty result rather than hanging.
            let (tx, rx) = channel();
            let _ = tx.send(Event::Done(Stats::default()));
            return Self { cancel, events: rx };
        }
        Self { cancel, events }
    }

    pub fn cancel(&self) {
        self.cancel.store(true, Ordering::Relaxed);
    }
}

/// Synchronous search (used by the thread and by tests).
pub fn run(
    root: &Path,
    matcher: &Matcher,
    cancel: &AtomicBool,
    out: &Sender<Event>,
    wake: &dyn Fn(),
) -> Stats {
    let mut stats = Stats::default();
    let mut stack = vec![root.to_path_buf()];
    let mut seen_files = 0;
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();
        entries.sort_by_key(|e| e.file_name());
        let mut subdirs = Vec::new();
        for entry in entries {
            if cancel.load(Ordering::Relaxed) || stats.hits >= MAX_HITS || seen_files >= MAX_FILES {
                stats.truncated = true;
                return stats;
            }
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with('.') {
                continue;
            }
            // symlink_metadata: do not follow links into other trees.
            let Ok(meta) = std::fs::symlink_metadata(entry.path()) else {
                continue;
            };
            if meta.is_dir() {
                if !SKIP_DIRS.contains(&name.as_ref()) {
                    subdirs.push(entry.path());
                }
                continue;
            }
            if !meta.is_file() {
                continue;
            }
            seen_files += 1;
            if meta.len() > MAX_FILE_BYTES {
                stats.files_skipped += 1;
                continue;
            }
            let path = entry.path();
            let Ok(bytes) = std::fs::read(&path) else {
                stats.files_skipped += 1;
                continue;
            };
            if bytes[..bytes.len().min(8192)].contains(&0) {
                stats.files_skipped += 1;
                continue;
            }
            let Ok(text) = std::str::from_utf8(&bytes) else {
                stats.files_skipped += 1;
                continue;
            };
            stats.files_searched += 1;
            let hits = file_hits(&path, text, matcher, MAX_HITS - stats.hits);
            if !hits.is_empty() {
                stats.hits += hits.len();
                if out.send(Event::Hits(hits)).is_err() {
                    return stats;
                }
                wake();
            }
        }
        // Depth-first, in name order.
        stack.extend(subdirs.into_iter().rev());
    }
    stats
}

fn file_hits(path: &Path, text: &str, matcher: &Matcher, limit: usize) -> Vec<Hit> {
    let mut hits = Vec::new();
    let (mut line, mut line_start) = (0usize, 0usize);
    let (mut scanned, mut chars) = (0usize, 0usize);
    for m in matcher.find_in_str(text).take(limit) {
        // Advance line and char counters to the match start.
        for (i, c) in text[scanned..m.start].char_indices() {
            if c == '\n' {
                line += 1;
                line_start = scanned + i + 1;
            }
        }
        chars += text[scanned..m.start].chars().count();
        scanned = m.start;
        let line_end = text[line_start..]
            .find(['\n', '\r'])
            .map_or(text.len(), |e| line_start + e);
        let match_end = m.end.min(line_end);
        let before: String = text[line_start..m.start].chars().collect();
        let before_chars = before.chars().count();
        // Keep the match visible: trim long prefixes.
        let skip = before_chars.saturating_sub(60);
        let mut preview: String = text[line_start..line_end]
            .chars()
            .skip(skip)
            .take(MAX_LINE_CHARS)
            .collect();
        let start = before_chars - skip;
        let len = text[m.start..match_end].chars().count();
        if preview.chars().count() < start + len {
            preview = text[line_start..line_end].chars().skip(skip).collect();
        }
        let match_chars = text[m.start..m.end].chars().count();
        hits.push(Hit {
            path: path.to_path_buf(),
            line,
            preview: preview.trim_end().to_owned(),
            preview_match: start..start + len,
            chars: chars..chars + match_chars,
        });
    }
    hits
}

#[cfg(test)]
mod tests {
    use super::*;
    use text3_core::search::Query;

    fn search(root: &Path, pattern: &str) -> (Vec<Hit>, Stats) {
        let m = Query {
            pattern: pattern.into(),
            ..Default::default()
        }
        .compile()
        .unwrap()
        .unwrap();
        let (tx, rx) = channel();
        let stats = run(root, &m, &AtomicBool::new(false), &tx, &|| {});
        drop(tx);
        let hits = rx
            .into_iter()
            .flat_map(|e| match e {
                Event::Hits(h) => h,
                Event::Done(_) => vec![],
            })
            .collect();
        (hits, stats)
    }

    #[test]
    fn finds_lines_and_positions_and_skips_noise() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path();
        std::fs::write(p.join("a.txt"), "first\nä needle here\r\nlast needle\n").unwrap();
        std::fs::create_dir(p.join("sub")).unwrap();
        std::fs::write(p.join("sub/b.md"), "no match\n").unwrap();
        std::fs::create_dir(p.join(".git")).unwrap();
        std::fs::write(p.join(".git/x"), "needle\n").unwrap();
        std::fs::create_dir(p.join("target")).unwrap();
        std::fs::write(p.join("target/y"), "needle\n").unwrap();
        std::fs::write(p.join("bin.dat"), b"needle\0\x01").unwrap();
        std::fs::write(p.join("latin.txt"), b"needle caf\xe9").unwrap();
        let (hits, stats) = search(p, "needle");
        assert_eq!(hits.len(), 2);
        assert_eq!(
            (hits[0].line, hits[0].preview.as_str()),
            (1, "ä needle here")
        );
        assert_eq!(hits[0].preview_match, 2..8);
        assert_eq!(hits[0].chars, 8..14);
        assert_eq!((hits[1].line, hits[1].chars.clone()), (2, 26..32));
        assert_eq!(stats.files_searched, 2);
        assert_eq!(stats.files_skipped, 2, "binary and non-UTF-8");
        assert!(!stats.truncated);
    }

    #[cfg(unix)]
    #[test]
    fn does_not_follow_directory_symlinks() {
        let d = tempfile::tempdir().unwrap();
        std::fs::write(d.path().join("a.txt"), "needle\n").unwrap();
        std::os::unix::fs::symlink(d.path(), d.path().join("loop")).unwrap();
        let (hits, _) = search(d.path(), "needle");
        assert_eq!(hits.len(), 1);
    }

    #[test]
    fn cancel_stops_early() {
        let d = tempfile::tempdir().unwrap();
        for i in 0..50 {
            std::fs::write(d.path().join(format!("{i}.txt")), "needle\n").unwrap();
        }
        let m = Query {
            pattern: "needle".into(),
            ..Default::default()
        }
        .compile()
        .unwrap()
        .unwrap();
        let (tx, _rx) = channel();
        let stats = run(d.path(), &m, &AtomicBool::new(true), &tx, &|| {});
        assert!(stats.truncated);
        assert_eq!(stats.hits, 0);
    }
}
