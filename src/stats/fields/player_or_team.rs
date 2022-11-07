use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerOrTeam {
    #[default]
    #[serde(rename = "P")]
    Player,

    #[serde(rename = "T")]
    Team,
}
