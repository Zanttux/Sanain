//! Recently opened files, most recent first, kept in the config folder
//! (`recent`, one path per line). Missing files are skipped when listed.

use std::fs;
use std::path::{Path, PathBuf};

/// How many files are remembered.
pub const MAX: usize = 20;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Recent {
    paths: Vec<PathBuf>,
}

impl Recent {
    pub fn load(dir: &Path) -> Self {
        let text = fs::read_to_string(dir.join("recent")).unwrap_or_default();
        Self::decode(&text)
    }

    pub fn decode(text: &str) -> Self {
        let mut r = Self::default();
        for line in text.lines().filter(|l| !l.is_empty()) {
            let p = PathBuf::from(line);
            if !r.paths.contains(&p) && r.paths.len() < MAX {
                r.paths.push(p);
            }
        }
        r
    }

    pub fn encode(&self) -> String {
        let mut out = String::new();
        for p in &self.paths {
            if let Some(s) = p.to_str().filter(|s| !s.contains(['\n', '\r'])) {
                out.push_str(s);
                out.push('\n');
            }
        }
        out
    }

    pub fn save(&self, dir: &Path) -> std::io::Result<()> {
        fs::create_dir_all(dir)?;
        sanain_io::write_atomic(&dir.join("recent"), self.encode().as_bytes(), None).map(|_| ())
    }

    /// Move `path` to the front.
    pub fn push(&mut self, path: &Path) {
        let path = path.canonicalize().unwrap_or_else(|_| path.to_owned());
        self.paths.retain(|p| *p != path);
        self.paths.insert(0, path);
        self.paths.truncate(MAX);
    }

    /// Remembered files that still exist, most recent first.
    pub fn existing(&self) -> Vec<PathBuf> {
        self.paths.iter().filter(|p| p.is_file()).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn most_recent_first_without_duplicates() {
        let d = tempfile::tempdir().unwrap();
        let (a, b) = (d.path().join("a.txt"), d.path().join("b.txt"));
        fs::write(&a, "a").unwrap();
        fs::write(&b, "b").unwrap();
        let mut r = Recent::default();
        r.push(&a);
        r.push(&b);
        r.push(&a);
        let names: Vec<_> = r
            .existing()
            .iter()
            .map(|p| p.file_name().unwrap().to_owned())
            .collect();
        assert_eq!(names, vec!["a.txt", "b.txt"]);
        r.save(d.path()).unwrap();
        assert_eq!(Recent::load(d.path()), r);
        fs::remove_file(&b).unwrap();
        assert_eq!(r.existing().len(), 1, "missing files are skipped");
        for i in 0..30 {
            r.push(&d.path().join(format!("{i}")));
        }
        assert_eq!(r.paths.len(), MAX);
        assert_eq!(Recent::decode("x\n\nx\ny\n").paths.len(), 2);
    }
}
