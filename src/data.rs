//! This module handles gathering information to display to the user.

use crate::{api::Data as ApiData, config::Config};
use enum_dispatch::enum_dispatch;
use lolfetch_color::ColoredString;
use summoner::Info as SummonerInfo;

pub mod champions;
pub mod mastery;
pub mod match_history;
pub mod summoner;

pub struct ProcessedData {
    pub image_url: String,
    pub sections: Vec<Sections>,
}

impl ProcessedData {
    pub fn process(data: ApiData, config: &Config) -> Self {
        let mut sections = Vec::new();

        // Process the data based on the display mode.
        match &config.mode {
            _ => {}
        }

        Self {
            image_url: data.image_url,
            sections,
        }
    }
}

/// Trait that defines the information that can be displayed.
#[enum_dispatch]
pub trait SectionInfoProvider {
    /// Returns the header of the section, not all sections have a header.
    fn header(&self) -> Option<String>;

    /// Returns the body of the section.
    fn body(&self) -> Vec<ColoredString>;
}

/// Enum that represents the different sections that can be displayed.
#[enum_dispatch(SectionInfoProvider)]
pub enum Sections {
    SummonerInfo,
}
