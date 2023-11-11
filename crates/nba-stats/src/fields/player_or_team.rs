use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Player {
    #[default]
    #[serde(rename = "Player")]
    Player,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Team {
    #[default]
    #[serde(rename = "Team")]
    Team,
}
