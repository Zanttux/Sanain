//! Application state for Sanain, independent of the GUI toolkit.

pub mod editor;
pub mod recovery;

pub use editor::{Command, Editor, SaveOutcome};
