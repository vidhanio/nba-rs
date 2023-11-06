use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Sorter {
    #[default]
    #[serde(rename = "DATE")]
    Date,
}
