use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DraftPick {
    #[serde(rename = "1st Round")]
    Round1,

    #[serde(rename = "2nd Round")]
    Round2,

    #[serde(rename = "1st Pick")]
    First,

    #[serde(rename = "Lottery Pick")]
    Lottery,

    #[serde(rename = "Top 5 Pick")]
    Top5,

    #[serde(rename = "Top 10 Pick")]
    Top10,

    #[serde(rename = "Top 15 Pick")]
    Top15,

    #[serde(rename = "Top 20 Pick")]
    Top20,

    #[serde(rename = "Top 25 Pick")]
    Top25,

    #[serde(rename = "Picks 11 Thru 20")]
    From11Thru20,

    #[serde(rename = "Picks 21 Thru 30")]
    From21Thru30,

    #[serde(rename = "Undrafted")]
    Undrafted,
}
