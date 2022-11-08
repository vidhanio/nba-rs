use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    #[serde(rename = "W")]
    Win,

    #[serde(rename = "L")]
    Loss,
}
