use crate::data::{
    champion_stats::RecentChampionInfo, mastery::Mastery, match_history::MatchHistory,
    summoner::Summoner, ApplicationData,
};
use anyhow::Result;
use enum_dispatch::enum_dispatch;
use lolfetch_color::ColoredString;
use std::io::Write;
use termcolor::{Buffer, BufferWriter, ColorChoice};

pub mod utils;

pub const CENTER_PAD_LENGTH: usize = 5;
pub const IMAGE_WIDTH: u32 = 50;
pub const IMAGE_HEIGHT: u32 = 25;

/// Trait that defines a displayable section.
#[enum_dispatch]
pub trait DisplayableSection {
    /// Returns the header of the section, not all sections have a header.
    fn header(&self) -> Option<String>;

    /// Returns the body of the section.
    fn body(&self) -> Vec<ColoredString>;
}

/// Enum that defines the different kinds of displayable sections.
#[enum_dispatch(DisplayableSection)]
pub enum DisplayableSectionKind {
    Summoner,
    MatchHistory,
    RecentChampionInfo,
    Mastery,
}

impl DisplayableSectionKind {
    pub fn to_colored_string_vec(&self) -> Vec<ColoredString> {
        let mut vec = Vec::new();

        if let Some(header) = self.header() {
            vec.push(ColoredString::from_str(&header, None, None));
            vec.push(ColoredString::from_str(
                "-".repeat(CENTER_PAD_LENGTH).as_str(),
                None,
                None,
            ));
        }

        self.body().iter().for_each(|body| {
            vec.push(body.clone());
        });

        vec
    }
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

pub struct Layout {
    processed: ApplicationData,
}

impl Layout {
    pub const fn new(processed: ApplicationData) -> Self {
        Self { processed }
    }

    pub fn display(&self) -> Result<()> {
        let mut logo_lines = self.processed.image.iter();
        let mut info_lines = Vec::new();

        for (i, section) in self.processed.sections.iter().enumerate() {
            if i != 0 {
                info_lines.push(ColoredString::from_str("", None, None));
            }
            info_lines.extend(section.to_colored_string_vec());
        }

        let mut info_lines = info_lines.iter();

        let buffer = BufferWriter::stdout(ColorChoice::Always);

        if logo_lines.len() > info_lines.len() {
            let diff = (logo_lines.len() - info_lines.len()) / 2;
            for (i, logo_line) in logo_lines.enumerate() {
                if i < diff {
                    buffer.print(&Self::format_line(Some(logo_line), None)?)?;
                } else {
                    buffer.print(&Self::format_line(Some(logo_line), info_lines.next())?)?;
                }
            }
        } else {
            for info_line in info_lines {
                buffer.print(&Self::format_line(logo_lines.next(), Some(info_line))?)?;
            }
        }

        Ok(())
    }

    fn format_line(
        logo_line: Option<&ColoredString>,
        info_line: Option<&ColoredString>,
    ) -> Result<Buffer> {
        let mut buffer = BufferWriter::stdout(ColorChoice::Always).buffer();

        match logo_line {
            Some(logo) => logo.display(&mut buffer)?,
            None => buffer.write_all(" ".repeat(IMAGE_WIDTH as usize).as_bytes())?,
        }

        if let Some(info) = info_line {
            buffer.write_all(" ".repeat(CENTER_PAD_LENGTH).as_bytes())?;
            info.display(&mut buffer)?;
        }

        buffer.write_all(b"\n")?;

        Ok(buffer)
    }
}
