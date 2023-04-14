use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClutchTime {
    #[default]
    #[serde(rename = "Last 5 Minutes")]
    Last5Minutes,

    #[serde(rename = "Last 4 Minutes")]
    Last4Minutes,

    #[serde(rename = "Last 3 Minutes")]
    Last3Minutes,

    #[serde(rename = "Last 2 Minutes")]
    Last2Minutes,

    #[serde(rename = "Last 1 Minute")]
    Last1Minute,

    #[serde(rename = "Last 30 Seconds")]
    Last30Seconds,

    #[serde(rename = "Last 10 Seconds")]
    Last10Seconds,
}
