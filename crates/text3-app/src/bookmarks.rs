//! Security-scoped bookmarks for the App Store build (#79). The sandbox
//! lets the app use a file only after the user chose it; a bookmark kept
//! in the config folder (`bookmarks`) brings that access back after a
//! restart, so Open Recent, session restore and hot exit work there.
//! Only the store build uses it: the other builds are not sandboxed.
//!
//! File format: the header line, then one bookmark per line, most recent
//! first: the bookmark bytes in hex, a tab, the path.

use std::fs;
use std::path::{Path, PathBuf};

const HEADER: &str = "text3-bookmarks 1";
const FILE: &str = "bookmarks";
/// More than Open Recent remembers, so open tabs fit as well.
pub const MAX: usize = 64;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Bookmarks {
    entries: Vec<(PathBuf, Vec<u8>)>,
}

impl Bookmarks {
    pub fn load(dir: &Path) -> Self {
        Self::decode(&fs::read_to_string(dir.join(FILE)).unwrap_or_default())
    }

    pub fn decode(text: &str) -> Self {
        let mut lines = text.lines();
        let mut b = Self::default();
        if lines.next() != Some(HEADER) {
            return b;
        }
        for line in lines {
            let Some((hex, path)) = line.split_once('\t') else {
                continue;
            };
            if let Some(data) = from_hex(hex).filter(|d| !d.is_empty())
                && !path.is_empty()
                && b.entries.len() < MAX
            {
                let path = PathBuf::from(path);
                if !b.entries.iter().any(|(p, _)| *p == path) {
                    b.entries.push((path, data));
                }
            }
        }
        b
    }

    pub fn encode(&self) -> String {
        let mut out = format!("{HEADER}\n");
        for (path, data) in &self.entries {
            if let Some(s) = path.to_str().filter(|s| !s.contains(['\n', '\r'])) {
                out.push_str(&to_hex(data));
                out.push('\t');
                out.push_str(s);
                out.push('\n');
            }
        }
        out
    }

    pub fn save(&self, dir: &Path) -> std::io::Result<()> {
        fs::create_dir_all(dir)?;
        text3_io::write_atomic(&dir.join(FILE), self.encode().as_bytes(), None).map(|_| ())
    }

    /// Put the bookmark for `path` first, replacing an older one.
    pub fn set(&mut self, path: &Path, data: Vec<u8>) {
        self.entries.retain(|(p, _)| p != path);
        self.entries.insert(0, (path.to_owned(), data));
        self.entries.truncate(MAX);
    }

    pub fn paths(&self) -> impl Iterator<Item = &Path> {
        self.entries.iter().map(|(p, _)| p.as_path())
    }

    /// Open every bookmark with `open` (normally
    /// [`text3_platform::open_bookmark`]). Bookmarks that no longer
    /// resolve are dropped and stale ones replaced. Returns true when
    /// something changed and the file should be saved.
    pub fn open_all(&mut self, open: impl Fn(&[u8]) -> Option<(PathBuf, Option<Vec<u8>>)>) -> bool {
        let before = self.entries.clone();
        let mut kept = Vec::with_capacity(self.entries.len());
        for (_, data) in self.entries.drain(..) {
            if let Some((path, fresh)) = open(&data)
                && !kept.iter().any(|(p, _)| *p == path)
            {
                kept.push((path, fresh.unwrap_or(data)));
            }
        }
        self.entries = kept;
        self.entries != before
    }
}

/// At startup of the store build: get access to the remembered files
/// again, before the session and hot exit reopen them.
pub fn restore(dir: &Path) {
    if !crate::STORE {
        return;
    }
    let mut b = Bookmarks::load(dir);
    if b.open_all(text3_platform::open_bookmark) {
        let _ = b.save(dir);
    }
}

/// In the store build, remember access to `path`, a file the user just
/// opened or saved.
pub fn remember(dir: &Path, path: &Path) {
    if !crate::STORE {
        return;
    }
    let mut b = Bookmarks::load(dir);
    if b.entries.first().is_some_and(|(p, _)| p == path) {
        return;
    }
    if let Some(data) = text3_platform::bookmark(path) {
        b.set(path, data);
        let _ = b.save(dir);
    }
}

fn to_hex(data: &[u8]) -> String {
    use std::fmt::Write;
    data.iter()
        .fold(String::with_capacity(data.len() * 2), |mut s, b| {
            let _ = write!(s, "{b:02x}");
            s
        })
}

fn from_hex(s: &str) -> Option<Vec<u8>> {
    if !s.len().is_multiple_of(2) || !s.is_ascii() {
        return None;
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(s: &str) -> PathBuf {
        PathBuf::from(s)
    }

    #[test]
    fn round_trips_most_recent_first() {
        let mut b = Bookmarks::default();
        b.set(&p("/a.txt"), vec![1, 2]);
        b.set(&p("/c d/ä.md"), vec![0xff]);
        b.set(&p("/a.txt"), vec![3]);
        let back = Bookmarks::decode(&b.encode());
        assert_eq!(back, b);
        assert_eq!(
            back.paths().collect::<Vec<_>>(),
            [Path::new("/a.txt"), Path::new("/c d/ä.md")]
        );
        assert!(b.encode().starts_with("text3-bookmarks 1\n03\t/a.txt\n"));
    }

    #[test]
    fn bad_lines_and_headers_are_ignored() {
        assert_eq!(Bookmarks::decode("other 1\n01\t/a\n"), Bookmarks::default());
        let b = Bookmarks::decode("text3-bookmarks 1\nzz\t/a\n0\t/b\n\t/c\n01\t\n01\n02\t/d\n");
        assert_eq!(b.paths().collect::<Vec<_>>(), [Path::new("/d")]);
    }

    #[test]
    fn keeps_at_most_max() {
        let mut b = Bookmarks::default();
        for i in 0..MAX + 5 {
            b.set(&p(&format!("/{i}")), vec![1]);
        }
        assert_eq!(b.paths().count(), MAX);
        assert_eq!(b.paths().next(), Some(Path::new(&format!("/{}", MAX + 4))));
    }

    #[test]
    fn open_all_drops_dead_and_refreshes_stale() {
        let mut b = Bookmarks::default();
        b.set(&p("/gone"), vec![0]);
        b.set(&p("/moved"), vec![1]);
        b.set(&p("/same"), vec![2]);
        let changed = b.open_all(|d| match d {
            [1] => Some((p("/new place"), Some(vec![9]))),
            [2] => Some((p("/same"), None)),
            _ => None,
        });
        assert!(changed);
        assert_eq!(
            b.entries,
            vec![(p("/same"), vec![2]), (p("/new place"), vec![9])]
        );
        assert!(!b.open_all(|d| Some((
            if d == [2] {
                p("/same")
            } else {
                p("/new place")
            },
            None
        ))));
    }

    #[test]
    fn saves_in_the_config_dir() {
        let d = tempfile::tempdir().unwrap();
        let mut b = Bookmarks::default();
        b.set(&d.path().join("x"), vec![7, 8]);
        b.save(&d.path().join("cfg")).unwrap();
        assert_eq!(Bookmarks::load(&d.path().join("cfg")), b);
    }

    #[test]
    fn other_builds_never_touch_the_file() {
        if crate::STORE {
            return;
        }
        let d = tempfile::tempdir().unwrap();
        fs::write(d.path().join("x"), "x").unwrap();
        remember(d.path(), &d.path().join("x"));
        restore(d.path());
        assert!(!d.path().join(FILE).exists());
    }
}
