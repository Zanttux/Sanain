//! Crash recovery snapshots for unsaved buffers.
//!
//! One file per buffer in a local directory. Each file is written atomically
//! and carries a format version and checksum, so a crash mid-write or a
//! damaged file is detected instead of restored as garbage. Snapshots hold
//! user text in plain form; they are not encrypted.
//!
//! File layout (all integers little-endian):
//! `magic "SANAINRC"` · `u32 version` · `u32 header_len` · header ·
//! `u64 text_len` · text (UTF-8) · `u64 FNV-1a of everything before`.
//! The header is UTF-8 `key=value` lines: `encoding`, optional `path`,
//! `saved` (Unix seconds).

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::encoding::TextEncoding;
use crate::file::write_atomic;

const MAGIC: &[u8; 8] = b"SANAINRC";
const VERSION: u32 = 1;
const EXTENSION: &str = "sanain-recovery";
/// Refuse to read absurdly large or hostile files.
const MAX_FILE: u64 = 1 << 32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snapshot {
    /// The file the buffer belongs to; `None` for an untitled buffer.
    pub path: Option<PathBuf>,
    pub encoding: TextEncoding,
    pub text: String,
}

#[derive(Debug)]
pub struct Entry {
    pub id: String,
    pub saved: SystemTime,
    pub snapshot: Snapshot,
}

#[derive(Debug, thiserror::Error)]
pub enum RecoveryError {
    #[error("recovery I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("recovery file {file} is damaged: {reason}")]
    Corrupt { file: PathBuf, reason: &'static str },
}

#[derive(Debug, Clone)]
pub struct RecoveryStore {
    dir: PathBuf,
}

impl RecoveryStore {
    /// Use `dir`, creating it (owner-only on Unix) if needed.
    pub fn open(dir: impl Into<PathBuf>) -> io::Result<Self> {
        let dir = dir.into();
        fs::create_dir_all(&dir)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&dir, fs::Permissions::from_mode(0o700))?;
        }
        Ok(Self { dir })
    }

    /// The platform's per-user location, e.g.
    /// `~/Library/Application Support/Sanain/Recovery` on macOS.
    pub fn default_dir() -> Option<PathBuf> {
        let env = |k: &str| {
            std::env::var_os(k)
                .filter(|v| !v.is_empty())
                .map(PathBuf::from)
        };
        if cfg!(target_os = "macos") {
            env("HOME").map(|h| h.join("Library/Application Support/Sanain/Recovery"))
        } else if cfg!(windows) {
            env("LOCALAPPDATA").map(|d| d.join("Sanain").join("Recovery"))
        } else {
            env("XDG_DATA_HOME")
                .or_else(|| env("HOME").map(|h| h.join(".local/share")))
                .map(|d| d.join("sanain").join("recovery"))
        }
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// A new identifier, unique across processes and restarts.
    pub fn new_id() -> String {
        use std::sync::atomic::{AtomicU32, Ordering};
        static N: AtomicU32 = AtomicU32::new(0);
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos());
        format!(
            "{t:x}-{:x}-{:x}",
            std::process::id(),
            N.fetch_add(1, Ordering::Relaxed)
        )
    }

    fn file_for(&self, id: &str) -> PathBuf {
        self.dir.join(format!("{id}.{EXTENSION}"))
    }

    pub fn write(&self, id: &str, snapshot: &Snapshot) -> io::Result<()> {
        let bytes = encode(snapshot, SystemTime::now());
        #[cfg(unix)]
        let perms = {
            use std::os::unix::fs::PermissionsExt;
            Some(fs::Permissions::from_mode(0o600))
        };
        #[cfg(not(unix))]
        let perms = None;
        write_atomic(&self.file_for(id), &bytes, perms).map(|_| ())
    }

    /// Remove a snapshot. Missing snapshots are not an error.
    pub fn remove(&self, id: &str) -> io::Result<()> {
        match fs::remove_file(self.file_for(id)) {
            Err(e) if e.kind() != io::ErrorKind::NotFound => Err(e),
            _ => Ok(()),
        }
    }

    /// All snapshots, newest first. Damaged files are returned as errors
    /// and left on disk for inspection.
    pub fn list(&self) -> io::Result<Vec<Result<Entry, RecoveryError>>> {
        let mut out = Vec::new();
        for item in fs::read_dir(&self.dir)? {
            let path = item?.path();
            if path.extension().and_then(|e| e.to_str()) != Some(EXTENSION) {
                continue;
            }
            let Some(id) = path.file_stem().and_then(|s| s.to_str()).map(str::to_owned) else {
                continue;
            };
            out.push(read_file(&path).map(|(saved, snapshot)| Entry {
                id,
                saved,
                snapshot,
            }));
        }
        out.sort_by_key(|r| std::cmp::Reverse(r.as_ref().map(|e| e.saved).ok()));
        Ok(out)
    }
}

fn read_file(path: &Path) -> Result<(SystemTime, Snapshot), RecoveryError> {
    let corrupt = |reason| RecoveryError::Corrupt {
        file: path.to_owned(),
        reason,
    };
    if fs::metadata(path)?.len() > MAX_FILE {
        return Err(corrupt("too large"));
    }
    let bytes = fs::read(path)?;
    decode(&bytes).map_err(corrupt)
}

fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(0x0100_0000_01b3)
    })
}

fn encoding_name(e: TextEncoding) -> &'static str {
    match e {
        TextEncoding::Utf8 => "utf-8",
        TextEncoding::Utf8Bom => "utf-8-bom",
        TextEncoding::Utf16LeBom => "utf-16le-bom",
        TextEncoding::Utf16BeBom => "utf-16be-bom",
        TextEncoding::Legacy(e) => e.name(),
    }
}

fn encode(s: &Snapshot, saved: SystemTime) -> Vec<u8> {
    let secs = saved.duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs());
    let mut header = format!("encoding={}\nsaved={secs}\n", encoding_name(s.encoding));
    // Paths that are not valid UTF-8 are not recorded; the text is still
    // recovered as an untitled buffer.
    if let Some(p) = s.path.as_ref().and_then(|p| p.to_str())
        && !p.contains('\n')
    {
        header.push_str(&format!("path={p}\n"));
    }
    let mut out = Vec::with_capacity(32 + header.len() + s.text.len());
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&(header.len() as u32).to_le_bytes());
    out.extend_from_slice(header.as_bytes());
    out.extend_from_slice(&(s.text.len() as u64).to_le_bytes());
    out.extend_from_slice(s.text.as_bytes());
    let sum = fnv1a(&out);
    out.extend_from_slice(&sum.to_le_bytes());
    out
}

fn decode(bytes: &[u8]) -> Result<(SystemTime, Snapshot), &'static str> {
    let (body, sum) = bytes.split_last_chunk::<8>().ok_or("truncated")?;
    if fnv1a(body) != u64::from_le_bytes(*sum) {
        return Err("checksum mismatch");
    }
    let rest = body.strip_prefix(MAGIC).ok_or("not a recovery file")?;
    let (ver, rest) = rest.split_first_chunk::<4>().ok_or("truncated")?;
    if u32::from_le_bytes(*ver) != VERSION {
        return Err("unsupported version");
    }
    let (hlen, rest) = rest.split_first_chunk::<4>().ok_or("truncated")?;
    let hlen = u32::from_le_bytes(*hlen) as usize;
    let header = rest.get(..hlen).ok_or("truncated")?;
    let header = std::str::from_utf8(header).map_err(|_| "bad header")?;
    let rest = &rest[hlen..];
    let (tlen, rest) = rest.split_first_chunk::<8>().ok_or("truncated")?;
    let tlen = usize::try_from(u64::from_le_bytes(*tlen)).map_err(|_| "bad length")?;
    if rest.len() != tlen {
        return Err("length mismatch");
    }
    let text = String::from_utf8(rest.to_vec()).map_err(|_| "text is not UTF-8")?;

    let (mut encoding, mut path, mut saved) = (None, None, UNIX_EPOCH);
    for line in header.lines() {
        match line.split_once('=') {
            Some(("encoding", v)) => {
                encoding = Some(match v {
                    "utf-8" => TextEncoding::Utf8,
                    "utf-8-bom" => TextEncoding::Utf8Bom,
                    "utf-16le-bom" => TextEncoding::Utf16LeBom,
                    "utf-16be-bom" => TextEncoding::Utf16BeBom,
                    other => match encoding_rs::Encoding::for_label(other.as_bytes()) {
                        Some(e) => TextEncoding::Legacy(e),
                        None => return Err("unknown encoding"),
                    },
                });
            }
            Some(("path", v)) => path = Some(PathBuf::from(v)),
            Some(("saved", v)) => {
                let secs: u64 = v.parse().map_err(|_| "bad time")?;
                saved = UNIX_EPOCH + std::time::Duration::from_secs(secs);
            }
            _ => {} // unknown keys are ignored for forward compatibility
        }
    }
    let encoding = encoding.ok_or("missing encoding")?;
    Ok((
        saved,
        Snapshot {
            path,
            encoding,
            text,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Snapshot {
        Snapshot {
            path: Some(PathBuf::from("/tmp/notes ä.txt")),
            encoding: TextEncoding::Utf16LeBom,
            text: "line\r\n🙂 é\n".into(),
        }
    }

    #[test]
    fn write_list_remove_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let store = RecoveryStore::open(dir.path().join("rec")).unwrap();
        let untitled = Snapshot {
            path: None,
            encoding: TextEncoding::Utf8,
            text: String::new(),
        };
        store.write("a", &sample()).unwrap();
        store.write("b", &untitled).unwrap();
        store.write("a", &sample()).unwrap(); // overwrite in place
        let mut got: Vec<_> = store
            .list()
            .unwrap()
            .into_iter()
            .map(|r| r.unwrap())
            .collect();
        got.sort_by(|x, y| x.id.cmp(&y.id));
        assert_eq!(got.len(), 2);
        assert_eq!(got[0].snapshot, sample());
        assert_eq!(got[1].snapshot, untitled);
        store.remove("a").unwrap();
        store.remove("a").unwrap(); // idempotent
        assert_eq!(store.list().unwrap().len(), 1);
    }

    #[test]
    fn every_single_byte_corruption_is_detected() {
        let bytes = encode(&sample(), UNIX_EPOCH);
        assert!(decode(&bytes).is_ok());
        for i in 0..bytes.len() {
            let mut b = bytes.clone();
            b[i] ^= 0x01;
            assert!(decode(&b).is_err(), "flip at byte {i} not detected");
        }
        for n in 0..bytes.len() {
            assert!(decode(&bytes[..n]).is_err(), "truncation to {n} accepted");
        }
    }

    #[test]
    fn damaged_file_is_reported_and_kept() {
        let dir = tempfile::tempdir().unwrap();
        let store = RecoveryStore::open(dir.path()).unwrap();
        store.write("ok", &sample()).unwrap();
        let bad = dir.path().join("bad.sanain-recovery");
        fs::write(&bad, b"SANAINRC garbage").unwrap();
        fs::write(dir.path().join("ignored.txt"), b"x").unwrap();
        let list = store.list().unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(list.iter().filter(|r| r.is_err()).count(), 1);
        assert!(bad.exists());
    }

    #[cfg(unix)]
    #[test]
    fn snapshots_are_private_to_the_user() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let store = RecoveryStore::open(dir.path().join("r")).unwrap();
        store.write("x", &sample()).unwrap();
        let mode = |p: &Path| fs::metadata(p).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode(store.dir()), 0o700);
        assert_eq!(mode(&store.dir().join("x.sanain-recovery")), 0o600);
    }
}
