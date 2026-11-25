//! UI-independent document model.
//!
//! A [`Document`] owns a rope buffer and an undo history. Every mutation goes
//! through a [`Transaction`], which records enough information to be inverted
//! exactly. Positions are expressed in Unicode scalar values (`char`
//! indices); callers convert to bytes or graphemes explicitly.

mod document;
mod transaction;

pub use document::{Document, LineEnding, Revision};
pub use transaction::{Change, EditError, Transaction};
