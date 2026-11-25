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

    let bytes = encode(text, encoding);
    let dir = target
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let mut tmp = tempfile::Builder::new()
        .prefix(".sanain-save-")
        .tempfile_in(dir)
        .map_err(io_err)?;
    tmp.write_all(&bytes).map_err(io_err)?;
    if existing.is_some() {
        let perms = fs::metadata(&target).map_err(io_err)?.permissions();
        fs::set_permissions(tmp.path(), perms).map_err(io_err)?;
    }
    tmp.as_file().sync_all().map_err(io_err)?;
    // On failure the temp file is removed and the original is untouched.
    tmp.persist(&target).map_err(|e| io_err(e.error))?;
    sync_dir(dir);

    let meta = fs::metadata(&target).map_err(io_err)?;
    Ok(Fingerprint::of(&bytes, &meta))
}

fn resolve_symlink(path: &Path) -> io::Result<PathBuf> {
    match fs::symlink_metadata(path) {
        Ok(m) if m.file_type().is_symlink() => fs::canonicalize(path),
        _ => Ok(path.to_owned()),
    }
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
