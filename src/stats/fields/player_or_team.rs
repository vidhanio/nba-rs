use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerOrTeam {
    #[default]
    #[serde(rename = "Player")]
    Player,

    #[serde(rename = "Team")]
    Team,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerOrTeamChar {
    #[default]
    #[serde(rename = "P")]
    Player,

    #[serde(rename = "T")]
    Team,
}

super::convert! {
    PlayerOrTeamChar => PlayerOrTeam {
        Player,
        Team,
    }
}
