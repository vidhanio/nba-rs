use serde::{Deserialize, Serialize};

pub mod basic;

/// The standard response from the NBA Stats API.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Response<P, R> {
    /// The parameters used to generate the response.
    pub parameters: P,

    /// The resource accessed.
    pub resource: String,

    /// The result sets returned.
    #[serde(alias = "resultSet")]
    pub result_sets: R,
}
