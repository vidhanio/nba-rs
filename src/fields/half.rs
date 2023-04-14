use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Half {
    #[serde(rename = "First Half")]
    FirstHalf,

    #[serde(rename = "Second Half")]
    SecondHalf,

    #[serde(rename = "Overtime")]
    Overtime,
}
