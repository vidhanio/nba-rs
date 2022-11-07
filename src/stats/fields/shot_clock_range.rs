use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShotClockRange {
    #[serde(rename = "24-22")]
    From24To22,

    #[serde(rename = "22-18 Very Early")]
    From22To18VeryEarly,

    #[serde(rename = "18-15 Early")]
    From18To15Early,

    #[serde(rename = "15-7 Average")]
    From15To7Average,

    #[serde(rename = "7-4 Late")]
    From7To4Late,

    #[serde(rename = "4-0 Very Late")]
    From4To0VeryLate,
}
