use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Conference {
    #[serde(rename = "East")]
    East,

    #[serde(rename = "West")]
    West,
}
