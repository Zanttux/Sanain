//! The list of open files, restored on the next launch.
//!
//! Format (UTF-8 text): first line `sanain-session 1`, then `active=N` and
//! one `path=...` line per tab in order, then `pinned=I` for each pinned
//! tab (an index into the paths). Unknown lines are ignored. Paths
//! that are not valid UTF-8 or contain line breaks are not recorded.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const HEADER: &str = "sanain-session 1";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Session {
    pub paths: Vec<PathBuf>,
    /// Index into `paths` of the active tab.
    pub active: usize,
    /// Indices into `paths` of pinned tabs.
    pub pinned: Vec<usize>,
}

impl Session {
    pub fn encode(&self) -> String {
        let mut out = format!("{HEADER}\nactive={}\n", self.active);
        let mut written = 0;
        let mut pinned = Vec::new();
        for (i, p) in self.paths.iter().enumerate() {
            if let Some(s) = p.to_str().filter(|s| !s.contains(['\n', '\r'])) {
                if self.pinned.contains(&i) {
                    pinned.push(written);
                }
                written += 1;
                out.push_str("path=");
                out.push_str(s);
                out.push('\n');
            }
        }
        for i in pinned {
            out.push_str(&format!("pinned={i}\n"));
        }
        out
    }

    /// Parse a session file. Anything malformed yields `None`, never a panic.
    pub fn decode(text: &str) -> Option<Self> {
        let mut lines = text.lines();
        if lines.next()? != HEADER {
            return None;
        }
        let mut s = Session::default();
        for line in lines {
            match line.split_once('=') {
                Some(("active", v)) => s.active = v.parse().ok()?,
                Some(("path", v)) if !v.is_empty() => s.paths.push(PathBuf::from(v)),
                Some(("pinned", v)) => s.pinned.extend(v.parse::<usize>().ok()),
                _ => {}
            }
        }
        if s.active >= s.paths.len() {
            s.active = 0;
        }
        let n = s.paths.len();
        s.pinned.retain(|i| *i < n);
        Some(s)
    }

    pub fn load(file: &Path) -> Option<Self> {
        Self::decode(&fs::read_to_string(file).ok()?)
    }

    pub fn save(&self, file: &Path) -> io::Result<()> {
        if let Some(dir) = file.parent() {
            fs::create_dir_all(dir)?;
        }
        sanain_io::write_atomic(file, self.encode().as_bytes(), None).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_and_clamping() {
        let s = Session {
            paths: vec!["/a/b.txt".into(), "/c d/ä.md".into()],
            active: 1,
            pinned: vec![1],
        };
        assert_eq!(Session::decode(&s.encode()), Some(s));
        let bad_active = "sanain-session 1\nactive=9\npath=/x\n";
        assert_eq!(Session::decode(bad_active).unwrap().active, 0);
    }

    #[test]
    fn rejects_garbage_without_panicking() {
        for text in [
            "",
            "nope",
            "sanain-session 2\n",
            "sanain-session 1\nactive=x\n",
        ] {
            assert_eq!(Session::decode(text), None, "{text:?}");
        }
        let ok = Session::decode("sanain-session 1\nfuture=1\npath=/x\n").unwrap();
        assert_eq!(ok.paths, vec![PathBuf::from("/x")]);
    }

    #[test]
    fn save_and_load_file() {
        let dir = tempfile::tempdir().unwrap();
        let f = dir.path().join("sub/session");
        let s = Session {
            paths: vec![dir.path().join("x.txt")],
            active: 0,
            pinned: Vec::new(),
        };
        s.save(&f).unwrap();
        assert_eq!(Session::load(&f), Some(s));
    }
}
