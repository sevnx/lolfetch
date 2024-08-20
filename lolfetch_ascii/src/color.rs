//! Handles everything related to colors in the terminal

use std::{fmt, io::Write};
use termcolor::{Buffer, Color, ColorSpec, WriteColor};

#[derive(Debug, Clone, Copy)]
pub struct ColoredChar {
    pub character: char,
    pub color: Option<termcolor::Color>,
}

impl ColoredChar {
    #[must_use]
    pub const fn new(character: char, color: Option<termcolor::Color>) -> Self {
        Self { character, color }
    }

    pub fn write_to_buffer(&self, buffer: &mut Buffer) -> Result<(), fmt::Error> {
        if let Some(color) = self.color {
            buffer
                .set_color(ColorSpec::new().set_fg(Some(color)))
                .map_err(|_| fmt::Error)?;
        }
        write!(buffer, "{}", self.character).map_err(|_| fmt::Error)?;
        buffer.reset().map_err(|_| fmt::Error)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ColoredString {
    vec: Vec<ColoredChar>,
}

impl ColoredString {
    #[must_use]
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    pub fn from_str(s: &str, color: Option<Color>) -> Self {
        let mut vec = Vec::new();
        for c in s.chars() {
            vec.push(ColoredChar::new(c, color));
        }
        Self { vec }
    }

    pub fn push(&mut self, item: ColoredChar) {
        self.vec.push(item);
    }

    pub fn push_str(&mut self, item: &str, color: Option<Color>) {
        for c in item.chars() {
            self.push(ColoredChar::new(c, color));
        }
    }

    pub fn iter(&self) -> std::slice::Iter<ColoredChar> {
        self.vec.iter()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

impl From<Vec<ColoredChar>> for ColoredString {
    fn from(vec: Vec<ColoredChar>) -> Self {
        Self { vec }
    }
}
