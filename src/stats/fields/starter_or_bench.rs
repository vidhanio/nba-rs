use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StarterOrBench {
    #[serde(rename = "Starters")]
    Starters,

    #[serde(rename = "Bench")]
    Bench,
}
