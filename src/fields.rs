//! All possible fields which may be used in the request or response of a
//! [`crate::stats::Endpoint`].

#![allow(missing_docs)]

use std::fmt::{self, Debug, Formatter};

use serde::{Deserialize, Serialize};

pub mod team;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum LeagueId {
    #[default]
    #[serde(rename = "00")]
    Nba,

    #[serde(rename = "01")]
    Aba,

    #[serde(rename = "20")]
    GLeague,

    #[serde(rename = "10")]
    Wnba,
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

    #[serde(rename = "Pre Season")]
    PreSeason,

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
pub enum YesNo {
    #[default]
    #[serde(rename = "N")]
    No,

    #[serde(rename = "Y")]
    Yes,
}

impl From<YesNo> for bool {
    fn from(value: YesNo) -> Self {
        match value {
            YesNo::No => false,
            YesNo::Yes => true,
        }
    }
}

impl From<bool> for YesNo {
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
pub enum Position {
    #[default]
    #[serde(rename = "C")]
    Center,

    #[serde(rename = "C-F")]
    CenterForward,

    #[serde(rename = "F")]
    Forward,

    #[serde(rename = "F-G")]
    ForwardGuard,

    #[serde(rename = "G")]
    Guard,
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
