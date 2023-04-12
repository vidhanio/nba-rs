pub(crate) mod macros;

use std::borrow::Cow;

use async_trait::async_trait;
use reqwest::{Client, Request};
use serde::{de::DeserializeOwned, Serialize};

use crate::{BasicResponse, Response, Result, CLIENT};

/// A trait which represents an endpoint in the NBA Stats API.
#[async_trait]
pub trait Endpoint: Sync {
    /// The type of the parameters used to generate the response.
    type Parameters: Serialize + DeserializeOwned;

    /// The type of the result sets returned by the endpoint.
    type ResultSets: DeserializeOwned;

    fn new(params: Self::Parameters) -> Self;

    /// The endpoint's name.
    fn endpoint(&self) -> Cow<'static, str>;

    /// The parameters used to generate the response.
    fn parameters(&self) -> Self::Parameters;

    /// Creates a [`reqwest::Request`] from `self` using the provided
    /// [`reqwest::Client`].
    fn to_request_with_client(&self, client: &Client) -> Request {
        client
            .get(format!("https://stats.nba.com/stats/{}", self.endpoint()))
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
        self.send_raw_with_client(client)
            .await?
            .json()
            .await
            .map_err(Into::into)
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
        client.execute(self.to_request()).await.map_err(Into::into)
    }
}
