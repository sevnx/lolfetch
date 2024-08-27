use crate::data::Sections;
use anyhow::Result;
use lolfetch_ascii::ColoredArt;
use lolfetch_color::ColoredString;
use std::io::Write;
use termcolor::{Buffer, BufferWriter, ColorChoice};

pub mod utils;

const CENTER_PAD_LENGTH: usize = 5;

pub struct Layout {
    image: ColoredArt,
    info: Vec<Sections>,
}

#[derive(Debug, Clone, Copy)]
enum Line<'a> {
    Logo(&'a ColoredString),
    Info(&'a ColoredString),
    Both(&'a ColoredString, &'a ColoredString),
}

trait ColoredStringDisplayer {
    fn display(&self, buffer: &mut Buffer) -> Result<()>;
}

impl ColoredStringDisplayer for ColoredString {
    fn display(&self, buffer: &mut Buffer) -> Result<()> {
        for char in self.iter() {
            char.write_to_buffer(buffer)?;
        }
        Ok(())
    }
}

impl Layout {
    pub fn new(logo: ColoredArt, info: Vec<Sections>) -> Self {
        Self { image: logo, info }
    }

    pub fn display(&self) -> Result<()> {
        let mut logo_lines = self.image.iter();
        let mut info_lines = Vec::new();

        for (i, section) in self.info.iter().enumerate() {
            if i != 0 {
                info_lines.push(ColoredString::from_str("", None, None));
            }
            let section_lines = section.to_colored_string_vec(CENTER_PAD_LENGTH);
            info_lines.extend(section_lines);
        }

        let mut info_lines = info_lines.iter();

        let buffer = BufferWriter::stdout(ColorChoice::Always);

        if logo_lines.len() > info_lines.len() {
            let diff = (logo_lines.len() - info_lines.len()) / 2;
            for (i, logo_line) in logo_lines.enumerate() {
                if i < diff {
                    buffer.print(&self.format_line(&Line::Logo(logo_line))?)?;
                } else {
                    match info_lines.next() {
                        Some(info_line) => {
                            buffer.print(&self.format_line(&Line::Both(logo_line, info_line))?)?;
                        }
                        None => {
                            buffer.print(&self.format_line(&Line::Logo(logo_line))?)?;
                        }
                    }
                }
            }
        } else {
            let diff = (info_lines.len() - logo_lines.len()) / 2;
            for (i, info_line) in info_lines.enumerate() {
                if i < diff {
                    buffer.print(&self.format_line(&Line::Info(info_line))?)?;
                } else {
                    let logo_line = logo_lines
                        .next()
                        .ok_or_else(|| anyhow::anyhow!("Logo lines ran out before info lines"))?;
                    buffer.print(&self.format_line(&Line::Both(logo_line, info_line))?)?;
                }
            }
        }

        Ok(())
    }

    fn format_line(&self, line: &Line) -> Result<Buffer> {
        let mut buffer = BufferWriter::stdout(ColorChoice::Always).buffer();

        match line {
            Line::Logo(logo) => {
                logo.display(&mut buffer)?;
            }
            Line::Info(info) => {
                let logo_width = self.image.first().map(|line| line.len()).unwrap_or(0);
                buffer.write(" ".repeat(logo_width + CENTER_PAD_LENGTH).as_bytes())?;
                info.display(&mut buffer)?;
            }
            Line::Both(logo, info) => {
                logo.display(&mut buffer)?;
                buffer.write(" ".repeat(CENTER_PAD_LENGTH).as_bytes())?;
                info.display(&mut buffer)?;
            }
        }

        buffer.write("\n".as_bytes())?;

        Ok(buffer)
    }
}
