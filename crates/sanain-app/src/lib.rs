//! Application state for Sanain, independent of the GUI toolkit.

pub mod editor;
pub mod recovery;
pub mod session;
pub mod syntax;
pub mod workspace;

pub use editor::{Command, Editor, SaveOutcome};
