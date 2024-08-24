//! This is a library that provides utilities for working with colored characters and strings
//! meant to primarly be used to display it in the terminal.

mod color;

// Re-exporting the color module
pub use color::{ColoredChar, ColoredString};
