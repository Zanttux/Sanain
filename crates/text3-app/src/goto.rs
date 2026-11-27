//! Goto Anything (quick open): what the query asks for, the files it can
//! open and how they rank, and the background listing of the project
//! folder.
//!
//! The project folder is the folder of the last Find in Folder search,
//! otherwise the active file's folder; there is no other project concept.
//! The listing reuses [`filesearch::walk`]: hidden entries and build
//! folders are skipped, symbolic links are not followed, and it stops at
//! [`MAX_FILES`]. `.gitignore` is not read.

use std::ops::ControlFlow;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};

use crate::filesearch::{self, WalkEnd};
use crate::fuzzy;
use crate::workspace::TabId;

/// The most files listed from the project folder.
pub const MAX_FILES: usize = 50_000;
/// The most rows the popup shows for a query.
pub const MAX_RESULTS: usize = 500;
/// The most matching lines `#text` lists.
pub const MAX_LINE_HITS: usize = 500;
/// Files sent from the listing thread at a time.
const BATCH: usize = 512;

/// What a Goto Anything query asks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Query<'a> {
    /// Files whose name or path fuzzy-matches `text`, opened at `line`
    /// (0-based line and column) when one is given: `main.rs:12:5`.
    Files {
        text: &'a str,
        line: Option<(usize, usize)>,
    },
    /// `:12` or `:12:5` in the current file; `None` if not a number yet.
    Line(Option<(usize, usize)>),
    /// `@name`: functions, types and headings of the current file.
    Symbol(&'a str),
    /// `#text`: lines of the current file containing `text`.
    Text(&'a str),
}

/// Read a query. A leading `:`, `@` or `#` picks the mode (as in
/// Sublime Text); otherwise files, with an optional `:line[:column]`
/// suffix. A trailing `:` alone is ignored while the number is typed.
pub fn parse(query: &str) -> Query<'_> {
    let q = query.trim_start();
    if let Some(rest) = q.strip_prefix(':') {
        return Query::Line(crate::editor::parse_line_target(rest));
    }
    if let Some(rest) = q.strip_prefix('@') {
        return Query::Symbol(rest.trim());
    }
    if let Some(rest) = q.strip_prefix('#') {
        return Query::Text(rest);
    }
    let q = q.trim_end();
    let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    if let Some((head, tail)) = q.rsplit_once(':') {
        if tail.is_empty() {
            return Query::Files {
                text: head,
                line: None,
            };
        }
        if digits(tail) {
            if let Some((file, line)) = head.rsplit_once(':')
                && digits(line)
            {
                return Query::Files {
                    text: file,
                    line: crate::editor::parse_line_target(&format!("{line}:{tail}")),
                };
            }
            return Query::Files {
                text: head,
                line: crate::editor::parse_line_target(tail),
            };
        }
    }
    Query::Files {
        text: q,
        line: None,
    }
}

/// Where a file candidate comes from; open tabs rank first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    Tab(TabId),
    Recent,
    Folder,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    /// `None` for an untitled tab.
    pub path: Option<PathBuf>,
    /// The file name (or the tab's title).
    pub name: String,
    /// The path matched against: relative to the project folder when the
    /// file is inside it, otherwise the full path.
    pub rel: String,
    pub source: Source,
}

impl Candidate {
    /// The folder part of [`Self::rel`] (empty for the project folder
    /// itself).
    pub fn dir(&self) -> &str {
        let cut = self.rel.len() - self.rel.rsplit(['/', '\\']).next().map_or(0, str::len);
        self.rel[..cut].trim_end_matches(['/', '\\'])
    }
}

/// `path` as shown and matched: relative to `root` when inside it.
pub fn relative(path: &Path, root: Option<&Path>) -> String {
    root.and_then(|r| path.strip_prefix(r).ok())
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(path)
        .to_string_lossy()
        .into_owned()
}

/// Every candidate once, in rank order for an empty query: open tabs,
/// then recent files, then the project folder's files. A path seen
/// earlier is not repeated.
pub fn candidates(
    tabs: &[(TabId, Option<PathBuf>, String)],
    recent: &[PathBuf],
    root: Option<&Path>,
    files: &[ListedFile],
) -> Vec<Candidate> {
    let mut seen: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();
    let mut out = Vec::with_capacity(tabs.len() + recent.len() + files.len());
    let key = |p: &Path| p.canonicalize().unwrap_or_else(|_| p.to_owned());
    for (id, path, title) in tabs {
        let rel = match path {
            Some(p) => {
                let k = key(p);
                let rel = relative(&k, root);
                if !seen.insert(k) {
                    continue;
                }
                rel
            }
            None => title.clone(),
        };
        out.push(Candidate {
            path: path.clone(),
            name: title.clone(),
            rel,
            source: Source::Tab(*id),
        });
    }
    for p in recent {
        let k = key(p);
        let rel = relative(&k, root);
        if seen.insert(k) {
            out.push(Candidate {
                name: file_name(p),
                rel,
                path: Some(p.clone()),
                source: Source::Recent,
            });
        }
    }
    // Folder paths are already canonical (the root is) and unique;
    // compare without touching the disk again.
    for f in files {
        if !seen.contains(&f.path) {
            out.push(Candidate {
                name: file_name(&f.path),
                rel: f.rel.clone(),
                path: Some(f.path.clone()),
                source: Source::Folder,
            });
        }
    }
    out
}

fn file_name(p: &Path) -> String {
    p.file_name()
        .map_or_else(|| p.to_string_lossy(), |n| n.to_string_lossy())
        .into_owned()
}

/// Score of one candidate: the better of the path match and the file
/// name match (which counts more), plus a little for open and recent
/// files. A query with a path separator matches the path only.
pub fn score(query: &str, c: &Candidate) -> Option<i32> {
    let path = fuzzy::score(query, &c.rel);
    let name = if query.contains(['/', '\\']) {
        None
    } else {
        fuzzy::score(query, &c.name).map(|s| {
            // A name that starts with the query beats one that only
            // contains its letters.
            let prefix = c
                .name
                .to_lowercase()
                .starts_with(&query.trim().to_lowercase());
            s * 2 + if prefix { 10 } else { 0 }
        })
    };
    let best = path.max(name)?;
    Some(
        best + match c.source {
            Source::Tab(_) => 6,
            Source::Recent => 3,
            Source::Folder => 0,
        },
    )
}

/// Indices of the matching candidates, best first, at most
/// [`MAX_RESULTS`]. Ties go to the shorter path, then to the earlier
/// candidate; an empty query keeps the given order.
pub fn rank(query: &str, cands: &[Candidate]) -> Vec<usize> {
    if query.trim().is_empty() {
        return (0..cands.len().min(MAX_RESULTS)).collect();
    }
    let mut scored: Vec<(i32, usize, usize)> = cands
        .iter()
        .enumerate()
        .filter_map(|(i, c)| score(query, c).map(|s| (s, c.rel.len(), i)))
        .collect();
    scored.sort_unstable_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)));
    scored.truncate(MAX_RESULTS);
    scored.into_iter().map(|(_, _, i)| i).collect()
}

/// A line of the current file that contains the `#` text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineHit {
    /// 0-based.
    pub line: usize,
    pub preview: String,
    /// The match, as char indices into the file.
    pub chars: std::ops::Range<usize>,
}

/// Lines of `text` containing `needle` (case-insensitive, literal), at
/// most [`MAX_LINE_HITS`] matches.
pub fn find_lines(text: &str, needle: &str) -> Vec<LineHit> {
    let Ok(Some(matcher)) = text3_core::search::Query {
        pattern: needle.to_owned(),
        ..Default::default()
    }
    .compile() else {
        return Vec::new();
    };
    filesearch::file_hits(Path::new(""), text, &matcher, MAX_LINE_HITS)
        .into_iter()
        .map(|h| LineHit {
            line: h.line,
            preview: h.preview,
            chars: h.chars,
        })
        .collect()
}

/// A file found under the project folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListedFile {
    pub path: PathBuf,
    /// Relative to the folder.
    pub rel: String,
}

/// List the files under `root` (see the module docs), sending them in
/// batches. Returns how the listing ended.
pub fn list_files(
    root: &Path,
    max_files: usize,
    cancel: &AtomicBool,
    mut out: impl FnMut(Vec<ListedFile>) -> bool,
) -> WalkEnd {
    let mut batch = Vec::new();
    let end = filesearch::walk(root, max_files, cancel, |path, _| {
        batch.push(ListedFile {
            rel: relative(&path, Some(root)),
            path,
        });
        if batch.len() >= BATCH && !out(std::mem::take(&mut batch)) {
            return ControlFlow::Break(());
        }
        ControlFlow::Continue(())
    });
    if !batch.is_empty() {
        out(batch);
    }
    end
}

enum ScanEvent {
    Files(Vec<ListedFile>),
    Done(WalkEnd),
}

/// The project folder being listed on a background thread. Dropping it
/// stops the listing.
pub struct Scan {
    pub root: PathBuf,
    pub files: Vec<ListedFile>,
    /// How the listing ended; `None` while it runs.
    pub end: Option<WalkEnd>,
    /// Bumped whenever `files` grows (for caching rankings).
    pub generation: u64,
    cancel: Arc<AtomicBool>,
    events: Receiver<ScanEvent>,
}

impl std::fmt::Debug for Scan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scan")
            .field("root", &self.root)
            .field("files", &self.files.len())
            .field("end", &self.end)
            .finish_non_exhaustive()
    }
}

impl Drop for Scan {
    fn drop(&mut self) {
        self.cancel.store(true, Ordering::Relaxed);
    }
}

impl Scan {
    /// Start listing `root` (canonicalized, so paths compare with the
    /// open files'). `wake` is called when files arrive.
    pub fn start(root: &Path, max_files: usize, wake: impl Fn() + Send + 'static) -> Self {
        let root = root.canonicalize().unwrap_or_else(|_| root.to_owned());
        let cancel = Arc::new(AtomicBool::new(false));
        let (tx, events): (Sender<ScanEvent>, _) = channel();
        let flag = cancel.clone();
        let dir = root.clone();
        let spawned = std::thread::Builder::new()
            .name("text3-goto-files".into())
            .spawn(move || {
                let end = list_files(&dir, max_files, &flag, |batch| {
                    let sent = tx.send(ScanEvent::Files(batch)).is_ok();
                    wake();
                    sent
                });
                let _ = tx.send(ScanEvent::Done(end));
                wake();
            });
        let mut scan = Self {
            root,
            files: Vec::new(),
            end: None,
            generation: 0,
            cancel,
            events,
        };
        if spawned.is_err() {
            scan.end = Some(WalkEnd::Stopped);
        }
        scan
    }

    /// Take the files that arrived; whether anything changed.
    pub fn poll(&mut self) -> bool {
        let mut changed = false;
        while let Ok(e) = self.events.try_recv() {
            changed = true;
            match e {
                ScanEvent::Files(f) => {
                    self.files.extend(f);
                    self.generation += 1;
                }
                ScanEvent::Done(end) => self.end = Some(end),
            }
        }
        changed
    }

    pub fn cancel(&self) {
        self.cancel.store(true, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefixes_pick_the_mode() {
        assert_eq!(parse(":12"), Query::Line(Some((11, 0))));
        assert_eq!(parse(":12:5"), Query::Line(Some((11, 4))));
        assert_eq!(parse(":"), Query::Line(None));
        assert_eq!(parse(":x"), Query::Line(None));
        assert_eq!(parse("@ main "), Query::Symbol("main"));
        assert_eq!(parse("#needle"), Query::Text("needle"));
        assert_eq!(parse("# two words"), Query::Text(" two words"));
        assert_eq!(
            parse("main.rs"),
            Query::Files {
                text: "main.rs",
                line: None
            }
        );
    }

    #[test]
    fn file_with_line_and_column() {
        assert_eq!(
            parse("main.rs:12"),
            Query::Files {
                text: "main.rs",
                line: Some((11, 0))
            }
        );
        assert_eq!(
            parse("src/main.rs:12:5"),
            Query::Files {
                text: "src/main.rs",
                line: Some((11, 4))
            }
        );
        // Still typing the number.
        assert_eq!(
            parse("main.rs:"),
            Query::Files {
                text: "main.rs",
                line: None
            }
        );
        // A drive letter or other colon is part of the name.
        assert_eq!(
            parse(r"C:\x\a.txt"),
            Query::Files {
                text: r"C:\x\a.txt",
                line: None
            }
        );
        assert_eq!(
            parse("a:b"),
            Query::Files {
                text: "a:b",
                line: None
            }
        );
    }

    fn folder(rel: &str) -> Candidate {
        Candidate {
            path: Some(PathBuf::from("/p").join(rel)),
            name: rel.rsplit('/').next().unwrap().to_owned(),
            rel: rel.to_owned(),
            source: Source::Folder,
        }
    }

    #[test]
    fn ranking_prefers_names_prefixes_and_open_files() {
        let mut cands = vec![
            folder("docs/domain_notes.md"),
            folder("crates/app/src/main.rs"),
            folder("main/readme.txt"),
            folder("src/maintenance.rs"),
        ];
        let rel = |c: &[Candidate], r: Vec<usize>| -> Vec<String> {
            r.into_iter().map(|i| c[i].rel.clone()).collect()
        };
        let top = rel(&cands, rank("main.rs", &cands));
        assert_eq!(top[0], "crates/app/src/main.rs");
        assert_eq!(top[1], "src/maintenance.rs");
        // A folder in the query matches the path.
        let top = rel(&cands, rank("main/read", &cands));
        assert_eq!(top, vec!["main/readme.txt"]);
        // An open tab outranks the same name in the folder.
        cands.push(Candidate {
            source: Source::Tab(7),
            ..folder("other/main.rs")
        });
        assert_eq!(rank("main.rs", &cands)[0], 4);
        assert!(rank("zzz", &cands).is_empty());
        assert_eq!(rank("", &cands), vec![0, 1, 2, 3, 4], "given order");
        // Equal scores: the shorter path first.
        let c = vec![folder("aa/x.txt"), folder("x.txt")];
        assert_eq!(rank("x.txt", &c), vec![1, 0]);
    }

    #[test]
    fn results_are_capped() {
        let cands: Vec<Candidate> = (0..MAX_RESULTS + 50)
            .map(|i| folder(&format!("f{i}.txt")))
            .collect();
        assert_eq!(rank("f", &cands).len(), MAX_RESULTS);
        assert_eq!(rank("", &cands).len(), MAX_RESULTS);
    }

    #[test]
    fn candidates_merge_tabs_recent_and_folder_once() {
        let d = tempfile::tempdir().unwrap();
        let root = d.path().canonicalize().unwrap();
        let a = root.join("a.txt");
        let b = root.join("sub").join("b.txt");
        std::fs::create_dir(root.join("sub")).unwrap();
        std::fs::write(&a, "").unwrap();
        std::fs::write(&b, "").unwrap();
        let files = vec![
            ListedFile {
                path: a.clone(),
                rel: "a.txt".into(),
            },
            ListedFile {
                path: b.clone(),
                rel: relative(&b, Some(&root)),
            },
        ];
        let tabs = vec![
            (1, Some(a.clone()), "a.txt".to_owned()),
            (2, None, "Untitled".to_owned()),
        ];
        let c = candidates(&tabs, std::slice::from_ref(&b), Some(&root), &files);
        let got: Vec<(Source, &str)> = c.iter().map(|c| (c.source, c.rel.as_str())).collect();
        let sub_b = relative(&b, Some(&root));
        assert_eq!(
            got,
            vec![
                (Source::Tab(1), "a.txt"),
                (Source::Tab(2), "Untitled"),
                (Source::Recent, sub_b.as_str()),
            ]
        );
        assert_eq!(c[2].dir(), "sub");
        assert_eq!(c[0].dir(), "");
        assert_eq!(c[2].name, "b.txt");
    }

    #[test]
    fn find_lines_lists_matches_case_insensitively() {
        let hits = find_lines("one\nTwo two\nthree\n", "two");
        assert_eq!(hits.len(), 2);
        assert_eq!((hits[0].line, hits[0].chars.clone()), (1, 4..7));
        assert_eq!(hits[1].chars, 8..11);
        assert_eq!(hits[0].preview, "Two two");
        assert!(find_lines("abc", "").is_empty());
    }

    fn list(root: &Path, max: usize) -> (Vec<String>, WalkEnd) {
        let mut out = Vec::new();
        let end = list_files(root, max, &AtomicBool::new(false), |b| {
            out.extend(b.into_iter().map(|f| f.rel.replace('\\', "/")));
            true
        });
        (out, end)
    }

    #[test]
    fn listing_skips_hidden_and_build_folders() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path();
        for dir in [
            ".git",
            "node_modules",
            "target",
            "src",
            "src/.cache",
            ".hidden",
        ] {
            std::fs::create_dir_all(p.join(dir)).unwrap();
        }
        for f in [
            "a.txt",
            ".env",
            ".git/config",
            "node_modules/x.js",
            "target/out.o",
            "src/lib.rs",
            "src/.cache/c",
            ".hidden/h.txt",
        ] {
            std::fs::write(p.join(f), "x").unwrap();
        }
        let (files, end) = list(p, MAX_FILES);
        assert_eq!(files, vec!["a.txt", "src/lib.rs"]);
        assert_eq!(end, WalkEnd::Complete);
    }

    #[test]
    fn listing_stops_at_the_limit_and_when_cancelled() {
        let d = tempfile::tempdir().unwrap();
        for i in 0..(BATCH + 20) {
            std::fs::write(d.path().join(format!("{i:04}.txt")), "").unwrap();
        }
        let (files, end) = list(d.path(), 10);
        assert_eq!(files.len(), 10);
        assert_eq!(end, WalkEnd::Limit);
        let (files, end) = list(d.path(), MAX_FILES);
        assert_eq!(files.len(), BATCH + 20, "batches and the remainder");
        assert_eq!(end, WalkEnd::Complete);
        let end = list_files(d.path(), MAX_FILES, &AtomicBool::new(true), |_| true);
        assert_eq!(end, WalkEnd::Stopped);
        // The receiver going away stops the walk too.
        let mut batches = 0;
        let end = list_files(d.path(), MAX_FILES, &AtomicBool::new(false), |_| {
            batches += 1;
            false
        });
        assert_eq!((end, batches), (WalkEnd::Stopped, 1));
    }

    #[cfg(unix)]
    #[test]
    fn listing_does_not_follow_symlink_loops() {
        let d = tempfile::tempdir().unwrap();
        std::fs::write(d.path().join("a.txt"), "").unwrap();
        std::os::unix::fs::symlink(d.path(), d.path().join("loop")).unwrap();
        std::os::unix::fs::symlink(d.path().join("a.txt"), d.path().join("link.txt")).unwrap();
        let (files, _) = list(d.path(), MAX_FILES);
        assert_eq!(files, vec!["a.txt"]);
    }

    #[test]
    fn background_scan_delivers_files() {
        let d = tempfile::tempdir().unwrap();
        std::fs::write(d.path().join("a.txt"), "").unwrap();
        std::fs::create_dir(d.path().join("b")).unwrap();
        std::fs::write(d.path().join("b/c.txt"), "").unwrap();
        let mut scan = Scan::start(d.path(), MAX_FILES, || {});
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
        while scan.end.is_none() && std::time::Instant::now() < deadline {
            scan.poll();
            std::thread::sleep(std::time::Duration::from_millis(2));
        }
        assert_eq!(scan.end, Some(WalkEnd::Complete));
        assert_eq!(scan.files.len(), 2);
        assert!(scan.files[0].path.starts_with(&scan.root));
        assert!(scan.generation >= 1);
    }
}
