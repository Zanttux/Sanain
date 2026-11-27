//! Local history: a copy of each saved version of a file, kept on this
//! computer for a limited time so earlier versions can be compared or
//! restored.
//!
//! Layout: `<root>/<16 hex digits>/` per file (a hash of its path), holding
//! `path` (the file's path as UTF-8) and one `<unix millis>.v` file per
//! version with the exact saved bytes. Directories are owner-only on Unix
//! and versions are written atomically. Nothing is encrypted.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::file::write_atomic;

/// Files larger than this are not copied into history.
pub const MAX_FILE_BYTES: u64 = 10 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Retention {
    pub max_versions: usize,
    pub max_age: Duration,
}

impl Default for Retention {
    fn default() -> Self {
        Self {
            max_versions: 50,
            max_age: Duration::from_secs(30 * 24 * 3600),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub saved: SystemTime,
    pub size: u64,
    file: PathBuf,
}

impl Version {
    pub fn read(&self) -> io::Result<Vec<u8>> {
        fs::read(&self.file)
    }
}

#[derive(Debug, Clone)]
pub struct HistoryStore {
    root: PathBuf,
}

fn key(path: &Path) -> String {
    let canonical = path.canonicalize().unwrap_or_else(|_| path.to_owned());
    let bytes = canonical.to_string_lossy();
    let hash = bytes.bytes().fold(0xcbf2_9ce4_8422_2325u64, |h, b| {
        (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3)
    });
    format!("{hash:016x}")
}

#[cfg(unix)]
fn private_dir(dir: &Path) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(dir, fs::Permissions::from_mode(0o700))
}

#[cfg(not(unix))]
fn private_dir(_: &Path) -> io::Result<()> {
    Ok(())
}

#[cfg(unix)]
fn private_file() -> Option<fs::Permissions> {
    use std::os::unix::fs::PermissionsExt;
    Some(fs::Permissions::from_mode(0o600))
}

#[cfg(not(unix))]
fn private_file() -> Option<fs::Permissions> {
    None
}

impl HistoryStore {
    pub fn open(root: impl Into<PathBuf>) -> io::Result<Self> {
        let root = root.into();
        fs::create_dir_all(&root)?;
        private_dir(&root)?;
        Ok(Self { root })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    fn dir_for(&self, path: &Path) -> PathBuf {
        self.root.join(key(path))
    }

    /// Record `bytes` as the newest version of `path`, unless they equal
    /// the newest recorded version or exceed [`MAX_FILE_BYTES`]. Then apply
    /// `retention`. Returns whether a version was added.
    pub fn record(&self, path: &Path, bytes: &[u8], retention: Retention) -> io::Result<bool> {
        if bytes.len() as u64 > MAX_FILE_BYTES {
            return Ok(false);
        }
        let dir = self.dir_for(path);
        fs::create_dir_all(&dir)?;
        private_dir(&dir)?;
        if let Some(latest) = self.list(path)?.into_iter().next()
            && latest.size == bytes.len() as u64
            && latest.read()? == bytes
        {
            return Ok(false);
        }
        let name = path.canonicalize().unwrap_or_else(|_| path.to_owned());
        write_atomic(
            &dir.join("path"),
            name.to_string_lossy().as_bytes(),
            private_file(),
        )?;
        let mut millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_millis());
        // Two saves within the same millisecond get distinct names.
        while dir.join(format!("{millis}.v")).exists() {
            millis += 1;
        }
        write_atomic(&dir.join(format!("{millis}.v")), bytes, private_file())?;
        self.prune(path, retention)?;
        Ok(true)
    }

    /// Versions of `path`, newest first.
    pub fn list(&self, path: &Path) -> io::Result<Vec<Version>> {
        let dir = self.dir_for(path);
        let entries = match fs::read_dir(&dir) {
            Ok(e) => e,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(e) => return Err(e),
        };
        let mut out = Vec::new();
        for entry in entries {
            let entry = entry?;
            let name = entry.file_name();
            let Some(millis) = name
                .to_str()
                .and_then(|n| n.strip_suffix(".v"))
                .and_then(|n| n.parse::<u64>().ok())
            else {
                continue;
            };
            out.push(Version {
                saved: UNIX_EPOCH + Duration::from_millis(millis),
                size: entry.metadata()?.len(),
                file: entry.path(),
            });
        }
        out.sort_by_key(|v| std::cmp::Reverse(v.saved));
        Ok(out)
    }

    /// Remove versions beyond the retention limits.
    pub fn prune(&self, path: &Path, retention: Retention) -> io::Result<()> {
        let now = SystemTime::now();
        for (i, v) in self.list(path)?.into_iter().enumerate() {
            let too_old = now
                .duration_since(v.saved)
                .is_ok_and(|age| age > retention.max_age);
            if i >= retention.max_versions || too_old {
                fs::remove_file(&v.file)?;
            }
        }
        Ok(())
    }

    /// Delete all local history for every file.
    pub fn purge_all(&self) -> io::Result<()> {
        for entry in fs::read_dir(&self.root)? {
            let p = entry?.path();
            if p.is_dir() {
                fs::remove_dir_all(&p)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_lists_dedupes_and_prunes() {
        let d = tempfile::tempdir().unwrap();
        let store = HistoryStore::open(d.path().join("h")).unwrap();
        let file = d.path().join("doc.txt");
        fs::write(&file, "x").unwrap();
        let keep3 = Retention {
            max_versions: 3,
            ..Retention::default()
        };
        assert!(store.record(&file, b"v1", keep3).unwrap());
        assert!(
            !store.record(&file, b"v1", keep3).unwrap(),
            "same as newest"
        );
        for v in [b"v2", b"v3", b"v4"] {
            assert!(store.record(&file, v, keep3).unwrap());
        }
        let list = store.list(&file).unwrap();
        assert_eq!(list.len(), 3);
        assert_eq!(list[0].read().unwrap(), b"v4");
        assert_eq!(list[2].read().unwrap(), b"v2");
        let other = d.path().join("other.txt");
        assert!(store.list(&other).unwrap().is_empty());
        store.purge_all().unwrap();
        assert!(store.list(&file).unwrap().is_empty());
    }

    #[test]
    fn age_limit_and_size_limit() {
        let d = tempfile::tempdir().unwrap();
        let store = HistoryStore::open(d.path()).unwrap();
        let file = d.path().join("a.txt");
        let none = Retention {
            max_versions: 10,
            max_age: Duration::ZERO,
        };
        store.record(&file, b"old", Retention::default()).unwrap();
        std::thread::sleep(Duration::from_millis(5));
        store.prune(&file, none).unwrap();
        assert!(store.list(&file).unwrap().is_empty());
        let big = vec![b'x'; (MAX_FILE_BYTES + 1) as usize];
        assert!(!store.record(&file, &big, Retention::default()).unwrap());
    }

    #[cfg(unix)]
    #[test]
    fn history_is_private() {
        use std::os::unix::fs::PermissionsExt;
        let d = tempfile::tempdir().unwrap();
        let store = HistoryStore::open(d.path().join("h")).unwrap();
        let file = d.path().join("s.txt");
        store
            .record(&file, b"secret", Retention::default())
            .unwrap();
        let v = &store.list(&file).unwrap()[0];
        assert_eq!(
            fs::metadata(&v.file).unwrap().permissions().mode() & 0o777,
            0o600
        );
        assert_eq!(
            fs::metadata(store.root()).unwrap().permissions().mode() & 0o777,
            0o700
        );
    }
}
