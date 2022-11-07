use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeasonType {
    #[serde(rename = "Pre Season")]
    PreSeason,

    #[default]
    #[serde(rename = "Regular Season")]
    RegularSeason,

    #[serde(rename = "Playoffs")]
    Playoffs,

    #[serde(rename = "All Star")]
    AllStar,

    #[serde(rename = "PlayIn")]
    PlayIn,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeasonTypeWithoutPlayIn {
    #[serde(rename = "Pre Season")]
    PreSeason,

    #[default]
    #[serde(rename = "Regular Season")]
    RegularSeason,

    #[serde(rename = "Playoffs")]
    Playoffs,

    #[serde(rename = "All Star")]
    AllStar,
}

super::convert_subset! {
    SeasonTypeWithoutPlayIn => SeasonType {
        PreSeason,
        RegularSeason,
        Playoffs,
        AllStar,
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeasonTypeBasic {
    #[default]
    #[serde(rename = "Regular Season")]
    RegularSeason,

    #[serde(rename = "Playoffs")]
    Playoffs,
}

super::convert_subset! {
    SeasonTypeBasic => SeasonType {
        RegularSeason,
        Playoffs,
    }
}

super::convert_subset! {
    SeasonTypeBasic => SeasonTypeWithoutPlayIn {
        RegularSeason,
        Playoffs,
    }
}
