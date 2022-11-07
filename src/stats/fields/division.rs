
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Division {
    #[serde(rename = "Atlantic")]
    Atlantic,

    #[serde(rename = "Central")]
    Central,

    #[serde(rename = "Southeast")]
    Southeast,

    #[serde(rename = "Northwest")]
    Northwest,

    #[serde(rename = "Pacific")]
    Pacific,

    #[serde(rename = "Southwest")]
    Southwest,
}
