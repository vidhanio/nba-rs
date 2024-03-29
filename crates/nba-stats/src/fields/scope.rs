use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Scope {
    #[default]
    #[serde(rename = "S")]
    AllPlayers,

    #[serde(rename = "Rookies")]
    Rookies,
}
