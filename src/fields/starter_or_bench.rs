use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StarterOrBench {
    #[serde(rename = "Starters")]
    Starters,

    #[serde(rename = "Bench")]
    Bench,
}
