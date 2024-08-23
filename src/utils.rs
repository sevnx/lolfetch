//! Utility module

pub struct Kda(pub i32, pub i32, pub i32);

impl Kda {
    pub fn get_kda(&self) -> Option<f64> {
        let Self(kills, deaths, assists) = self;
        if *deaths == 0 {
            None
        } else {
            Some(f64::from(kills + assists) / f64::from(*deaths))
        }
    }

    pub fn add(&mut self, other: &Self) {
        let Self(kills, deaths, assists) = self;
        let Self(other_kills, other_deaths, other_assists) = other;
        *kills += other_kills;
        *deaths += other_deaths;
        *assists += other_assists;
    }
}

impl std::fmt::Display for Kda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(kills, deaths, assists) = self;
        write!(f, "{kills}/{deaths}/{assists}")
    }
}
