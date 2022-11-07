use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Location {
    #[serde(rename = "Home")]
    Home,

    #[serde(rename = "Road")]
    Road,
}
