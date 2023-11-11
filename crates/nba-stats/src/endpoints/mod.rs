pub mod alltimeleadersgrids;
pub mod assistleaders;
pub mod assisttracker;
pub mod leagueleaders;
pub mod playbyplayv2;
pub mod playergamelogs;

use std::{borrow::Cow, collections::HashMap};

use async_trait::async_trait;
pub use nba_macros::Endpoint;
use reqwest::{Client, Request, Url};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    serde_utils::one_or_many::OneOrMany, BasicResponse, BasicResultSet, Response, Result, CLIENT,
};

const BASE_URL: &str = "https://stats.nba.com/stats/";

/// A trait which represents an endpoint in the NBA Stats API.
#[async_trait]
pub trait Endpoint: Sync {
    /// The type of the parameters used to generate the response.
    type Parameters: Serialize + DeserializeOwned;

    /// The type of the result sets returned by the endpoint.
    type ResultSets: DeserializeOwned;

    /// The endpoint's path.
    fn path(&self) -> Cow<'static, str>;

    /// The parameters used to generate the response.
    fn parameters(&self) -> Self::Parameters;

    /// The endpoint's url.
    fn url(&self) -> Url {
        Url::parse(BASE_URL)
            .expect("base url should be well-formed")
            .join(&self.path())
            .expect("endpoint should join")
    }

    /// Creates a [`reqwest::Request`] from `self` using the provided
    /// [`reqwest::Client`].
    fn to_request_with_client(&self, client: &Client) -> Request {
        client
            .get(self.url())
            .query(&self.parameters())
            .build()
            .expect("request should build")
    }

    /// Creates a [`reqwest::Request`] from `self` using [`crate::CLIENT`].
    fn to_request(&self) -> Request {
        self.to_request_with_client(&CLIENT)
    }

    /// Sends a request to the endpoint and returns the response
    /// deserialized to [`Response<Self::Parameters,
    /// Self::ResultSets>`] using [`crate::CLIENT`].
    async fn send(&self) -> Result<Response<Self::Parameters, Self::ResultSets>> {
        self.send_with_client(&CLIENT).await
    }

    /// Sends a request to the endpoint and returns the response
    /// deserialized to [`Response<Self::Parameters,
    /// Self::ResultSets>`] using the provided [`reqwest::Client`].
    async fn send_with_client(
        &self,
        client: &Client,
    ) -> Result<Response<Self::Parameters, Self::ResultSets>> {
        let Response {
            resource,
            parameters,
            result_sets,
        } = self
            .send_raw_with_client(client)
            .await?
            .json::<Response<Self::Parameters, OneOrMany<BasicResultSet>>>()
            .await?;

        let result_sets = result_sets
            .into_iter()
            .map(|rs| {
                (
                    rs.name,
                    rs.row_set
                        .into_iter()
                        .map(|row| rs.headers.iter().zip(row).collect::<serde_json::Value>())
                        .collect::<serde_json::Value>(),
                )
            })
            .collect();

        Ok(Response {
            resource,
            parameters,
            result_sets: serde_json::from_value(result_sets)?,
        })
    }

    /// Sends a request to the endpoint and returns the response
    /// deserialized to [`BasicResponse`] using [`crate::CLIENT`].
    async fn send_basic(&self) -> Result<BasicResponse> {
        self.send_basic_with_client(&CLIENT).await
    }

    /// Sends a request to the endpoint and returns the response
    /// deserialized to [`BasicResponse`] using the provided
    /// [`reqwest::Client`].
    async fn send_basic_with_client(&self, client: &Client) -> Result<BasicResponse> {
        self.send_raw_with_client(client)
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// Sends a request to the endpoint and returns the response as a
    /// [`reqwest::Response`] using [`crate::CLIENT`].
    async fn send_raw(&self) -> Result<reqwest::Response> {
        self.send_raw_with_client(&CLIENT).await
    }

    /// Sends a request to the endpoint and returns the response as a
    /// [`reqwest::Response`] using the provided [`reqwest::Client`].
    async fn send_raw_with_client(&self, client: &Client) -> Result<reqwest::Response> {
        client
            .execute(self.to_request())
            .await
            .and_then(reqwest::Response::error_for_status)
            .map_err(Into::into)
    }
}

/// A basic implementation of [`Endpoint`] which can be used for loosely typed
/// access.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BasicEndpoint {
    endpoint: String,
    parameters: serde_json::Map<String, serde_json::Value>,
}

impl BasicEndpoint {
    /// Creates a new [`BasicEndpoint`] with the provided path and
    /// parameters.
    #[must_use]
    pub const fn new(
        endpoint: String,
        parameters: serde_json::Map<String, serde_json::Value>,
    ) -> Self {
        Self {
            endpoint,
            parameters,
        }
    }

    /// Creates a new [`BasicEndpoint`] with the provided endpoint and
    /// [`null`](serde_json::Value::Null) parameters.
    #[must_use]
    pub fn from_endpoint(endpoint: String) -> Self {
        Self::new(endpoint, serde_json::Map::new())
    }
}

#[async_trait]
impl Endpoint for BasicEndpoint {
    type Parameters = serde_json::Map<String, serde_json::Value>;
    type ResultSets = HashMap<String, Vec<serde_json::Map<String, serde_json::Value>>>;

    fn path(&self) -> Cow<'static, str> {
        self.endpoint.clone().into()
    }

    fn parameters(&self) -> Self::Parameters {
        self.parameters.clone()
    }
}
