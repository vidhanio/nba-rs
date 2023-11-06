use serde::{Deserialize, Serialize};

use crate::{serde::one_or_many::OneOrMany, Response};

pub type BasicResponse =
    Response<serde_json::Map<String, serde_json::Value>, OneOrMany<BasicResultSet>>;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct BasicResultSet {
    pub name: String,
    pub headers: Vec<String>,
    pub row_set: Vec<Vec<serde_json::Value>>,
}
