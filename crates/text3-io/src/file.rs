use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::encoding::{DecodeError, EncodeError, TextEncoding, decode, decode_as, encode};

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
    /// A character cannot be represented in the file's encoding. Nothing
    /// was written.
    #[error("{source}")]
    Unencodable { path: PathBuf, source: EncodeError },
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

/// Load with an encoding the user chose (e.g. "Reopen with Encoding").
pub fn load_as(path: &Path, encoding: TextEncoding) -> Result<Loaded, LoadError> {
    let io_err = |source| LoadError::Io {
        path: path.to_owned(),
        source,
    };
    let bytes = fs::read(path).map_err(io_err)?;
    let meta = fs::metadata(path).map_err(io_err)?;
    let text = decode_as(&bytes, encoding).map_err(|source| LoadError::Decode {
        path: path.to_owned(),
        source,
    })?;
    Ok(Loaded {
        text,
        encoding,
        fingerprint: Fingerprint::of(&bytes, &meta),
    })
}

pub fn load(path: &Path) -> Result<Loaded, LoadError> {
    load_with_bytes(path).map(|(l, _)| l)
}

/// [`load`], also returning the bytes that were read (one read, so the
/// bytes and the fingerprint describe the same file contents).
pub fn load_with_bytes(path: &Path) -> Result<(Loaded, Vec<u8>), LoadError> {
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
    let fingerprint = Fingerprint::of(&bytes, &meta);
    Ok((
        Loaded {
            text,
            encoding,
            fingerprint,
        },
        bytes,
    ))
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

    let bytes = encode(text, encoding).map_err(|e| SaveError::Unencodable {
        path: path.to_owned(),
        source: e,
    })?;
    let perms = match existing {
        Some(_) => Some(fs::metadata(&target).map_err(io_err)?.permissions()),
        None => None,
    };
    // A file with several hard links must keep its inode, or the other
    // names would still show the old contents: overwrite it in place,
    // behind a backup copy of the old contents.
    let meta = match &existing {
        Some(old) if hard_link_count(&target) > 1 => {
            write_in_place(&target, old, &bytes, perms).map_err(io_err)?
        }
        _ => write_atomic(&target, &bytes, perms).map_err(io_err)?,
    };
    Ok(Fingerprint::of(&bytes, &meta))
}

#[cfg(unix)]
fn hard_link_count(path: &Path) -> u64 {
    use std::os::unix::fs::MetadataExt;
    fs::metadata(path).map_or(1, |m| m.nlink())
}

/// Hard-link counts are not available on stable Rust elsewhere; those
/// platforms keep the atomic rename.
#[cfg(not(unix))]
fn hard_link_count(_: &Path) -> u64 {
    1
}

/// Overwrite `target` in place (same inode), after writing `old` to a
/// backup next to it. The backup is removed on success. If the overwrite
/// fails (e.g. the disk is full), the old contents and modification time
/// are written back in place, so the file is unchanged, the backup is
/// removed and the error keeps its kind. Only if that restore fails too
/// does the backup stay; the error then names it and is no longer
/// reported as e.g. disk-full, since the file is not unchanged.
fn write_in_place(
    target: &Path,
    old: &[u8],
    bytes: &[u8],
    perms: Option<fs::Permissions>,
) -> io::Result<fs::Metadata> {
    let name = target
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    // Never replace an existing file (e.g. the backup of an earlier failed
    // save, which may hold the only good copy).
    let backup = (0..100)
        .map(|i| {
            let suffix = if i == 0 {
                String::new()
            } else {
                format!("-{i}")
            };
            target.with_file_name(format!(".{name}.sanain-backup{suffix}"))
        })
        .find(|p| fs::symlink_metadata(p).is_err())
        .ok_or_else(|| io::Error::new(io::ErrorKind::AlreadyExists, "too many backups"))?;
    let modified = fs::metadata(target)?.modified().ok();
    write_atomic(&backup, old, perms)?;
    let overwrite = |data: &[u8]| -> io::Result<fs::File> {
        let mut f = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(target)?;
        f.write_all(data)?;
        f.sync_all()?;
        Ok(f)
    };
    match overwrite(bytes).and_then(|f| f.metadata()) {
        Ok(meta) => {
            let _ = fs::remove_file(&backup);
            Ok(meta)
        }
        Err(e) => {
            // Truncating freed the old contents' space, so they fit again.
            // Restoring the time too keeps the editor's fingerprint valid.
            let restored = overwrite(old).and_then(|f| {
                if let Some(t) = modified {
                    f.set_modified(t)?;
                }
                f.sync_all()
            });
            match restored {
                Ok(()) => {
                    let _ = fs::remove_file(&backup);
                    Err(e)
                }
                Err(_) => Err(io::Error::other(format!(
                    "{e}; the file may be incomplete and its previous contents are in {}",
                    backup.display()
                ))),
            }
        }
    }
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
