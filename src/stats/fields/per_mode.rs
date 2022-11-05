use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerMode48 {
    #[default]
    #[serde(rename = "Totals")]
    Totals,

    #[serde(rename = "PerGame")]
    PerGame,

    #[serde(rename = "Per48")]
    Per48Minutes,
}
