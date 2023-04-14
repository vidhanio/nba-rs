use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlayType {
    #[default]
    #[serde(rename = "Isolation")]
    Isolation,

    #[serde(rename = "Transition")]
    Transition,

    #[serde(rename = "PRBallHandler")]
    PickAndRollBallHandler,

    #[serde(rename = "PRRollMan")]
    PickAndRollRollMan,

    #[serde(rename = "Postup")]
    PostUp,

    #[serde(rename = "Spotup")]
    SpotUp,

    #[serde(rename = "Handoff")]
    Handoff,

    #[serde(rename = "Cut")]
    Cut,

    #[serde(rename = "OffScreen")]
    OffScreen,

    #[serde(rename = "OffRebound")]
    Putbacks,

    #[serde(rename = "Misc")]
    Misc,
}
