use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Experience {
    #[serde(rename = "Rookie")]
    Rookie,

    #[serde(rename = "Sophomore")]
    Sophomore,

    #[serde(rename = "Veteran")]
    Veteran,
}
