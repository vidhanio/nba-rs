use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub enum Half {
    #[serde(rename = "First Half")]
    FirstHalf,

    #[serde(rename = "Second Half")]
    SecondHalf,

    #[serde(rename = "Overtime")]
    Overtime,
}
