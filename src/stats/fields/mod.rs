//! All possible fields which may be used in the request or response of a
//! [`crate::stats::Endpoint`].

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

pub mod per_mode;

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
