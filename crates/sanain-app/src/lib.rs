//! Application state for Sanain, independent of the GUI toolkit.

pub mod editor;
pub mod filesearch;
pub mod fuzzy;
pub mod images;
pub mod keymap;
pub mod recent;
pub mod recovery;
pub mod session;
pub mod settings;
pub mod syntax;
pub mod update;
pub mod workspace;

pub use editor::{Command, Editor, SaveOutcome};
