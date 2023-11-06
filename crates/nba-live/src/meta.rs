use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

/// The metadata for the scoreboard.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Meta {
    /// The metadata version.
    pub version: u8,

    /// The `nba.cloud` request url.
    pub request: String,

    /// The time the data was last updated.
    #[serde(with = "crate::serde::meta_datetime")]
    pub time: PrimitiveDateTime,

    /// The status code of the request.
    #[serde(with = "http_serde::status_code")]
    pub code: StatusCode,
}
