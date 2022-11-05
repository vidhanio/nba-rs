//! All possible fields which may be used in the request or response of a
//! [`crate::stats::Endpoint`].

#![allow(missing_docs)]

use std::fmt::{self, Debug, Formatter};

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

pub mod team;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum LeagueId {
    #[default]
    #[serde(rename = "00")]
    Nba,

    #[serde(rename = "10")]
    Wnba,

    #[serde(rename = "20")]
    GLeague,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum PerMode36 {
    #[default]
    #[serde(rename = "Totals")]
    Totals,

    #[serde(rename = "Per36")]
    Per36,

    #[serde(rename = "PerGame")]
    PerGame,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum PerModeSimple {
    #[default]
    #[serde(rename = "Totals")]
    Totals,

    #[serde(rename = "PerGame")]
    PerGame,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum SeasonType {
    #[default]
    #[serde(rename = "Regular Season")]
    RegularSeason,

    #[serde(rename = "Playoffs")]
    Playoffs,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum PlayerOrTeam {
    #[default]
    #[serde(rename = "Team")]
    Team,

    #[serde(rename = "Player")]
    Player,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum YesOrNo {
    #[default]
    #[serde(rename = "N")]
    No,

    #[serde(rename = "Y")]
    Yes,
}

impl From<YesOrNo> for bool {
    fn from(value: YesOrNo) -> Self {
        match value {
            YesOrNo::No => false,
            YesOrNo::Yes => true,
        }
    }
}

impl From<bool> for YesOrNo {
    fn from(value: bool) -> Self {
        if value {
            Self::Yes
        } else {
            Self::No
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PerModeStatSimple {
    Totals(u32),

    PerGame(f64),
}

impl PerModeStatSimple {
    #[must_use]
    pub fn as_f64(self) -> f64 {
        match self {
            Self::Totals(v) => v.into(),
            Self::PerGame(v) => v,
        }
    }
}

impl Default for PerModeStatSimple {
    fn default() -> Self {
        Self::Totals(0)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PerModeStat36 {
    Totals(u32),

    Per36(f64),

    PerGame(f64),
}

impl PerModeStat36 {
    #[must_use]
    pub fn as_f64(self) -> f64 {
        match self {
            Self::Totals(v) => v.into(),
            Self::Per36(v) | Self::PerGame(v) => v,
        }
    }
}

impl Default for PerModeStat36 {
    fn default() -> Self {
        Self::Totals(0)
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum PlayerPosition {
    #[default]
    #[serde(rename = "C")]
    Center,

    #[serde(rename = "F")]
    Forward,

    #[serde(rename = "G")]
    Guard,

    #[serde(rename = "C-F")]
    CenterForward,

    #[serde(rename = "F-C")]
    ForwardCenter,

    #[serde(rename = "F-G")]
    ForwardGuard,

    #[serde(rename = "G-F")]
    GuardForward,
}

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JerseyNumber {
    #[default]
    #[serde(rename = "00")]
    DoubleZero,

    #[serde(with = "crate::sd::u32str")]
    Number(u32),
}

impl Debug for JerseyNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DoubleZero => "00".to_owned(),
            Self::Number(n) => n.to_string(),
        };

        f.debug_tuple("JerseyNumber").field(&s).finish()
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PlayerStat<S> {
    Player(S),

    #[default]
    Team,
}

impl<S> From<PlayerStat<S>> for Option<S> {
    fn from(value: PlayerStat<S>) -> Self {
        match value {
            PlayerStat::Player(stat) => Some(stat),
            PlayerStat::Team => None,
        }
    }
}

impl<S> From<Option<S>> for PlayerStat<S> {
    fn from(value: Option<S>) -> Self {
        value.map_or(Self::Team, Self::Player)
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum Division {
    #[default]
    #[serde(rename = "Atlantic")]
    Atlantic,

    #[serde(rename = "Central")]
    Central,

    #[serde(rename = "Southeast")]
    Southeast,

    #[serde(rename = "Northwest")]
    Northwest,

    #[serde(rename = "Pacific")]
    Pacific,

    #[serde(rename = "Southwest")]
    Southwest,
}

impl Division {
    #[must_use]
    pub const fn conference(self) -> Conference {
        match self {
            Self::Atlantic | Self::Central | Self::Southeast => Conference::East,
            Self::Northwest | Self::Pacific | Self::Southwest => Conference::West,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum Conference {
    #[default]
    #[serde(rename = "East")]
    East,

    #[serde(rename = "West")]
    West,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConferenceOrDivision {
    Conference(Conference),

    Division(Division),
}

impl ConferenceOrDivision {
    #[must_use]
    pub const fn conference(self) -> Conference {
        match self {
            Self::Conference(c) => c,
            Self::Division(d) => d.conference(),
        }
    }
}

impl Default for ConferenceOrDivision {
    fn default() -> Self {
        Self::Conference(Conference::default())
    }
}

impl From<Conference> for ConferenceOrDivision {
    fn from(value: Conference) -> Self {
        Self::Conference(value)
    }
}

impl From<Division> for ConferenceOrDivision {
    fn from(value: Division) -> Self {
        Self::Division(value)
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum StarterOrBench {
    #[default]
    #[serde(rename = "Bench")]
    Bench,

    #[serde(rename = "Starter")]
    Starter,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum SeasonSegment {
    #[default]
    #[serde(rename = "Pre All-Star")]
    PreAllStar,

    #[serde(rename = "Post All-Star")]
    PostAllStar,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum Month {
    #[default]
    #[serde(rename = "January")]
    January,

    #[serde(rename = "February")]
    February,

    #[serde(rename = "March")]
    March,

    #[serde(rename = "April")]
    April,

    #[serde(rename = "May")]
    May,

    #[serde(rename = "June")]
    June,

    #[serde(rename = "July")]
    July,

    #[serde(rename = "August")]
    August,

    #[serde(rename = "September")]
    September,

    #[serde(rename = "October")]
    October,

    #[serde(rename = "November")]
    November,

    #[serde(rename = "December")]
    December,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum FullSeasonSegment {
    #[default]
    AllSeasonSegments,

    LastNGames(u32),

    Month(Month),

    SeasonSegment(SeasonSegment),
}

impl Serialize for FullSeasonSegment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AllSeasonSegments => serializer.serialize_str("All Season Segments"),
            Self::LastNGames(n) => serializer.serialize_str(&format!("Last {} Games", n)),
            Self::Month(m) => m.serialize(serializer),
            Self::SeasonSegment(s) => s.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for FullSeasonSegment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FullSeasonSegmentVisitor;

        impl<'de> Visitor<'de> for FullSeasonSegmentVisitor {
            type Value = FullSeasonSegment;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
                formatter.write_str("a full season segment")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "All Season Segments" => Ok(Self::Value::AllSeasonSegments),
                    s if s.starts_with("Last ") && s.ends_with(" Games") => {
                        let n = s[5..s.len() - 6].parse().map_err(E::custom)?;
                        Ok(Self::Value::LastNGames(n))
                    }
                    s => {
                        let m = serde_json::from_str(s).map_err(E::custom)?;
                        Ok(Self::Value::Month(m))
                    }
                }
            }
        }

        deserializer.deserialize_str(FullSeasonSegmentVisitor)
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum PlayerExperience {
    #[default]
    #[serde(rename = "Rookie")]
    Rookie,

    #[serde(rename = "Sophomore")]
    Sophomore,

    #[serde(rename = "Veteran")]
    Veteran,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum WinOrLoss {
    #[default]
    #[serde(rename = "L")]
    Loss,

    #[serde(rename = "W")]
    Win,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum Location {
    #[default]
    #[serde(rename = "Home")]
    Home,

    #[serde(rename = "Road")]
    Road,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum GameScope {
    #[default]
    #[serde(rename = "Yesterday")]
    Yesterday,

    #[serde(rename = "Last 10")]
    Last10,
}
