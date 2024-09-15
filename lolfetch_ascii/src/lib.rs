//! This module contains logic for fetching ascii art either from a local file or from a URL.

mod image;

// Re-exporting the image functions
pub use image::ArtProcessingError;
pub use image::ColoredArt;
pub use image::{from_file_path, from_url};
