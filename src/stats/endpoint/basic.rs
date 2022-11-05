use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{BasicResultSet, Endpoint};

/// A basic endpoint which can be used to retrieve any result set.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BasicEndpoint {
    /// The name of the endpoint.
    pub endpoint: String,

    /// The parameters used to generate the response.
    pub parameters: serde_json::Map<String, serde_json::Value>,
}

impl Endpoint for BasicEndpoint {
    type Parameters = serde_json::Map<String, serde_json::Value>;
    type ResultSets = Vec<BasicResultSet>;

    fn endpoint(&self) -> Cow<'static, str> {
        self.endpoint.clone().into()
    }

    fn parameters(&self) -> &Self::Parameters {
        &self.parameters
    }
}
