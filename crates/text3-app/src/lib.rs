//! Application state for Text³, independent of the GUI toolkit.

pub mod cli;
pub mod counts;
pub mod editor;
pub mod editorconfig;
pub mod filesearch;
pub mod fonts;
pub mod fuzzy;
pub mod goto;
pub mod images;
pub mod keymap;
pub mod macros;
pub mod migrate;
pub mod npp_shortcuts;
pub mod panes;
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
