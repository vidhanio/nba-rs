use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DraftPick {
    #[serde(rename = "1st Round")]
    Pick1stRound,

    #[serde(rename = "2nd Round")]
    Pick2ndRound,

    #[serde(rename = "1st Pick")]
    Pick1stPick,

    #[serde(rename = "Lottery Pick")]
    LotteryPick,

    #[serde(rename = "Top 5 Pick")]
    Top5Pick,

    #[serde(rename = "Top 10 Pick")]
    Top10Pick,

    #[serde(rename = "Top 15 Pick")]
    Top15Pick,

    #[serde(rename = "Top 20 Pick")]
    Top20Pick,

    #[serde(rename = "Top 25 Pick")]
    Top25Pick,

    #[serde(rename = "Picks 11 Thru 20")]
    Picks11Thru20,

    #[serde(rename = "Picks 21 Thru 30")]
    Picks21Thru30,

    #[serde(rename = "Undrafted")]
    Undrafted,
}
