use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeGrouping {
    #[default]
    #[serde(rename = "offensive")]
    Offensive,

    #[serde(rename = "defensive")]
    Defensive,
}
