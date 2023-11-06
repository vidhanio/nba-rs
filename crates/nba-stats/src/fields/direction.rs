use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[default]
    #[serde(rename = "DESC")]
    Descending,

    #[serde(rename = "ASC")]
    Ascending,
}
