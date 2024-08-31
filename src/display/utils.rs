//! This module contains utilities regarding the display of the application.

use lolfetch_color::ColoredString;
use termcolor::Color;

pub mod colors;

/// Generate a two-part loading bar.
pub fn generate_loading_bar(
    filled_part: i32,
    unfilled_part: i32,
    width: i32,
    fill_color: termcolor::Color,
) -> ColoredString {
    let total = filled_part + unfilled_part;

    let mut bar = ColoredString::new();

    let percentage_filled = ((filled_part as f32 / total as f32) * width as f32).round() as i32;

    for i in 0..width {
        bar.push_str(
            " ",
            None,
            if i < percentage_filled {
                Some(fill_color)
            } else {
                Some(Color::White)
            },
        );
    }

    bar
}
