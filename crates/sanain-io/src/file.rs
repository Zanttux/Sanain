use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::encoding::{DecodeError, TextEncoding, decode, encode};

/// What the editor knew about a file's on-disk state.
///
/// The content hash is an in-process check only (std's SipHash with fixed
/// keys); it is not persisted and must not be treated as cryptographic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fingerprint {
    len: u64,
    modified: Option<SystemTime>,
    hash: u64,
}

impl Fingerprint {
    fn of(bytes: &[u8], meta: &fs::Metadata) -> Self {
        let mut h = DefaultHasher::new();
        bytes.hash(&mut h);
        Self {
            len: bytes.len() as u64,
            modified: meta.modified().ok(),
            hash: h.finish(),
        }
    }
}

#[derive(Debug)]
pub struct Loaded {
    pub text: String,
    pub encoding: TextEncoding,
    pub fingerprint: Fingerprint,
}

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("could not read {path}: {source}")]
    Io { path: PathBuf, source: io::Error },
    #[error("could not decode {path}: {source}")]
    Decode { path: PathBuf, source: DecodeError },
}

#[derive(Debug, thiserror::Error)]
pub enum SaveError {
    /// The file on disk differs from the version the document was based on.
    /// Nothing was written.
    #[error("{path} changed on disk since it was opened")]
    ExternalChange { path: PathBuf },
    /// The target is marked read-only. Nothing was written.
    #[error("{path} is read-only")]
    ReadOnly { path: PathBuf },
    #[error("could not save {path}: {source}")]
    Io { path: PathBuf, source: io::Error },
}

/// How `save` treats an existing file at the target path.
#[derive(Debug, Clone, Copy)]
pub enum SaveMode<'a> {
    /// The file must still match this fingerprint (normal Save).
    Expect(&'a Fingerprint),
    /// The file must not exist (Save As to a new path, recovery export).
    CreateNew,
    /// Replace whatever is there. Only after explicit user confirmation.
    Overwrite,
}

pub fn load(path: &Path) -> Result<Loaded, LoadError> {
    let io_err = |source| LoadError::Io {
        path: path.to_owned(),
        source,
    };
    let bytes = fs::read(path).map_err(io_err)?;
    let meta = fs::metadata(path).map_err(io_err)?;
    let (text, encoding) = decode(&bytes).map_err(|source| LoadError::Decode {
        path: path.to_owned(),
        source,
    })?;
    Ok(Loaded {
        text,
        encoding,
        fingerprint: Fingerprint::of(&bytes, &meta),
    })
}

/// Atomically write `text` to `path` and return the new fingerprint.
///
/// If `path` is a symlink, the link target is replaced and the link kept.
/// Existing permissions are copied to the new file where the platform allows.
/// Read-only targets are refused. Known limit: hard links are split (the
/// other names keep the old content) and Unix owner/group are not copied.
pub fn save(
    path: &Path,
    text: &str,
    encoding: TextEncoding,
    mode: SaveMode<'_>,
) -> Result<Fingerprint, SaveError> {
    let io_err = |source| SaveError::Io {
        path: path.to_owned(),
        source,
    };
    let target = resolve_symlink(path).map_err(io_err)?;
    let existing = match fs::read(&target) {
        Ok(bytes) => Some(bytes),
        Err(e) if e.kind() == io::ErrorKind::NotFound => None,
        Err(e) => return Err(io_err(e)),
    };
    let conflict = match (mode, &existing) {
        (SaveMode::Overwrite, _) | (SaveMode::CreateNew, None) => false,
        (SaveMode::CreateNew, Some(_)) | (SaveMode::Expect(_), None) => true,
        (SaveMode::Expect(expected), Some(bytes)) => {
            let meta = fs::metadata(&target).map_err(io_err)?;
            Fingerprint::of(bytes, &meta) != *expected
        }
    };
    if conflict {
        return Err(SaveError::ExternalChange {
            path: path.to_owned(),
        });
    }

    if let Some(meta) = existing.as_ref().and(fs::metadata(&target).ok())
        && meta.permissions().readonly()
    {
        return Err(SaveError::ReadOnly {
            path: path.to_owned(),
        });
    }

    let bytes = encode(text, encoding);
    let perms = match existing {
        Some(_) => Some(fs::metadata(&target).map_err(io_err)?.permissions()),
        None => None,
    };
    let meta = write_atomic(&target, &bytes, perms).map_err(io_err)?;
    Ok(Fingerprint::of(&bytes, &meta))
}

/// Write `bytes` to `target` via a synced temporary file in the same
/// directory and an atomic rename. Returns the new file's metadata, read
/// before the rename so nothing can fail after the file is replaced.
pub fn write_atomic(
    target: &Path,
    bytes: &[u8],
    permissions: Option<fs::Permissions>,
) -> io::Result<fs::Metadata> {
    let dir = target
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let tmp = TempFile::create_in(dir)?;
    let mut file = &tmp.file;
    file.write_all(bytes)?;
    if let Some(perms) = permissions {
        fs::set_permissions(&tmp.path, perms)?;
    }
    tmp.file.sync_all()?;
    let meta = tmp.file.metadata()?;
    tmp.persist(target)?;
    sync_dir(dir);
    Ok(meta)
}

/// A temporary file next to the target, removed on drop unless persisted.
/// Created with default permissions (0666 minus umask on Unix) so new files
/// get the same mode any other program would give them.
struct TempFile {
    path: PathBuf,
    file: fs::File,
    persisted: bool,
}

impl TempFile {
    fn create_in(dir: &Path) -> io::Result<Self> {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let nanos = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_or(0, |d| d.subsec_nanos());
        for _ in 0..16 {
            let n = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = dir.join(format!(
                ".sanain-save-{}-{n}-{nanos:x}.tmp",
                std::process::id()
            ));
            match fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
            {
                Ok(file) => {
                    return Ok(Self {
                        path,
                        file,
                        persisted: false,
                    });
                }
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(e),
            }
        }
        Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "could not create a unique temporary file",
        ))
    }

    /// Atomically replace `target`. `std::fs::rename` replaces existing
    /// files on Unix (rename(2)) and Windows (MoveFileExW with
    /// MOVEFILE_REPLACE_EXISTING).
    fn persist(mut self, target: &Path) -> io::Result<()> {
        fs::rename(&self.path, target)?;
        self.persisted = true;
        Ok(())
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        if !self.persisted {
            let _ = fs::remove_file(&self.path);
        }
    }
}

/// Follow symlinks without requiring the final target to exist, so saving
/// through a dangling link creates the target. Bounded to avoid loops.
fn resolve_symlink(path: &Path) -> io::Result<PathBuf> {
    let mut current = path.to_owned();
    for _ in 0..40 {
        match fs::symlink_metadata(&current) {
            Ok(m) if m.file_type().is_symlink() => {
                let link = fs::read_link(&current)?;
                current = match current.parent() {
                    Some(parent) if link.is_relative() => parent.join(link),
                    _ => link,
                };
            }
            _ => return Ok(current),
        }
    }
    Err(io::Error::other("too many levels of symbolic links"))
}

/// Make the rename durable. Best effort: not all platforms or filesystems
/// support syncing a directory handle.
fn sync_dir(dir: &Path) {
    #[cfg(unix)]
    if let Ok(d) = fs::File::open(dir) {
        let _ = d.sync_all();
    }
    #[cfg(not(unix))]
    let _ = dir;
}
