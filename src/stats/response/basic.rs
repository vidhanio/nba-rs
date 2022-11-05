use serde::{Deserialize, Serialize};

use crate::{serde::VecOrSingle, Response};

pub type BasicResponse =
    Response<serde_json::Map<String, serde_json::Value>, VecOrSingle<BasicResultSet>>;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct BasicResultSet {
    pub name: String,
    pub headers: Vec<String>,
    pub row_set: Vec<Vec<serde_json::Value>>,
}
