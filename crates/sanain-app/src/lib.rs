//! Application state for Sanain, independent of the GUI toolkit.

pub mod cli;
pub mod editor;
pub mod editorconfig;
pub mod filesearch;
pub mod fuzzy;
pub mod images;
pub mod keymap;
pub mod recent;
pub mod recovery;
pub mod replacefiles;
pub mod session;
pub mod settings;
pub mod shellreg;
pub mod syntax;
pub mod update;
pub mod workspace;

pub use editor::{Command, Editor, SaveOutcome};
