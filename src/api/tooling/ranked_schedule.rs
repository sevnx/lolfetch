//! This module handles the ranked schedule.

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Split {
    Split1,
    Split2,
    Split3,
}

struct SplitBoundary {
    pub start: u32,
    pub end: u32,
}

impl Split {
    fn from_major(major_version: u32) -> Option<Self> {
        const SPLIT_1: SplitBoundary = SplitBoundary { start: 0, end: 9 };
        const SPLIT_2: SplitBoundary = SplitBoundary { start: 10, end: 18 };
        const SPLIT_3: SplitBoundary = SplitBoundary { start: 19, end: 24 };

        match major_version {
            v if (SPLIT_1.start..=SPLIT_1.end).contains(&v) => Some(Self::Split1),
            v if (SPLIT_2.start..=SPLIT_2.end).contains(&v) => Some(Self::Split2),
            v if (SPLIT_3.start..=SPLIT_3.end).contains(&v) => Some(Self::Split3),
            _ => None,
        }
    }
}

/// Returns the split from a given patch.
/// It is based on the patch schedule for 3 splits per year
/// (the app only supports the current split currently).
/// This will be changed accordingly if the split schedule changes.
pub fn get_split_from_patch(patch: &str) -> Option<Split> {
    let parts: Vec<&str> = patch.split('.').collect();
    let major: u32 = parts.get(1)?.parse().ok()?;
    Split::from_major(major)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("14.1.0" => Some(Split::Split1) ; "early split 1")]
    #[test_case("14.9" => Some(Split::Split1) ; "late split 1")]
    #[test_case("14.18" => Some(Split::Split2) ; "end split 2")]
    #[test_case("14.24" => Some(Split::Split3) ; "end split 3")]
    #[test_case("14.25" => None ; "invalid patch")]
    fn test_get_split_from_patch(patch: &str) -> Option<Split> {
        get_split_from_patch(patch)
    }
}
