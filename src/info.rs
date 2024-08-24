//! This module handles gathering information to display to the user.

use champions::RecentChampionInfo;
use enum_dispatch::enum_dispatch;
use lolfetch_color::ColoredString;
use mastery::MasteryInfo;
use match_history::RecentMatchesInfo;
use summoner::SummonerInfo;

pub mod champions;
pub mod mastery;
pub mod match_history;
pub mod ranked;
pub mod summoner;

/// Trait that defines the information that can be displayed.
#[enum_dispatch]
pub trait SectionInfoProvider {
    /// Returns the header of the section, not all sections have a header.
    fn header(&self) -> Option<ColoredString>;

    /// Returns the body of the section.
    fn body(&self) -> Vec<ColoredString>;
}

/// Enum that represents the different sections that can be displayed.
#[enum_dispatch(SectionInfoProvider)]
pub enum Sections {
    SummonerInfo,
    RecentMatchesInfo,
    RecentChampionInfo,
    MasteryInfo,
}

impl Sections {
    pub fn to_colored_string_vec(&self, separator_size: usize) -> Vec<ColoredString> {
        let mut vec = Vec::new();

        if let Some(header) = self.header() {
            vec.push(header);
            vec.push(ColoredString::from_str(
                &"-".repeat(separator_size),
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
