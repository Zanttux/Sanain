//! Byte-exact file loading and atomic saving.
//!
//! Rules enforced here:
//! - Decoding is strict. Invalid input is reported, never replaced, so a
//!   later save cannot silently corrupt bytes.
//! - Text keeps its original line endings; this crate never rewrites them.
//! - Saving writes a temporary file in the target directory, flushes it and
//!   atomically renames it over the target.
//! - Saving refuses to overwrite a file that changed on disk since it was
//!   loaded, unless the caller explicitly opts in.

mod encoding;
mod file;
pub mod history;
pub mod logfile;
pub mod recovery;

pub use encoding::{
    DecodeError, EncodeError, LEGACY, TextEncoding, decode, decode_as, encode, guess, guess_head,
    is_confident,
};
pub use file::{
    Fingerprint, LoadError, Loaded, SaveError, SaveMode, load, load_as, save, write_atomic,
};
