pub(crate) mod macros;

use std::{borrow::Cow, future::Future, pin::Pin};

use reqwest::{Client, Request};
use serde::{de::DeserializeOwned, Serialize};

use crate::{BasicResponse, Response, Result, CLIENT};

type BoxedResultFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T>> + Send + 'a>>;

/// A trait which represents an endpoint in the NBA Stats API.
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
    fn send<'s, 'f>(&'s self) -> BoxedResultFuture<'f, Response<Self::Parameters, Self::ResultSets>>
    where
        's: 'f,
    {
        Box::pin(async move { self.send_with_client(&CLIENT).await })
    }

    /// Sends a request to the endpoint and returns the response
    /// deserialized to [`Response<Self::Parameters,
    /// Self::ResultSets>`] using the provided [`reqwest::Client`].
    fn send_with_client<'s, 'c, 'f>(
        &'s self,
        client: &'c Client,
    ) -> BoxedResultFuture<'f, Response<Self::Parameters, Self::ResultSets>>
    where
        's: 'f,
        'c: 'f,
    {
        Box::pin(async move {
            self.send_raw_with_client(client)
                .await?
                .json()
                .await
                .map_err(Into::into)
        })
    }

    /// Sends a request to the endpoint and returns the response
    /// deserialized to [`BasicResponse`] using [`crate::CLIENT`].
    fn send_basic<'s, 'f>(&'s self) -> BoxedResultFuture<'f, BasicResponse>
    where
        's: 'f,
    {
        Box::pin(async move { self.send_basic_with_client(&CLIENT).await })
    }

    /// Sends a request to the endpoint and returns the response
    /// deserialized to [`BasicResponse`] using the provided
    /// [`reqwest::Client`].
    fn send_basic_with_client<'s, 'c, 'f>(
        &'s self,
        client: &'c Client,
    ) -> BoxedResultFuture<'f, BasicResponse>
    where
        's: 'f,
        'c: 'f,
    {
        Box::pin(async move {
            self.send_raw_with_client(client)
                .await?
                .json()
                .await
                .map_err(Into::into)
        })
    }

    /// Sends a request to the endpoint and returns the response as a
    /// [`reqwest::Response`] using [`crate::CLIENT`].
    fn send_raw<'s, 'f>(&'s self) -> BoxedResultFuture<'f, reqwest::Response>
    where
        's: 'f,
    {
        Box::pin(async move { self.send_raw_with_client(&CLIENT).await })
    }

    /// Sends a request to the endpoint and returns the response as a
    /// [`reqwest::Response`] using the provided [`reqwest::Client`].
    fn send_raw_with_client<'s, 'c, 'f>(
        &'s self,
        client: &'c Client,
    ) -> BoxedResultFuture<'f, reqwest::Response>
    where
        's: 'f,
        'c: 'f,
    {
        Box::pin(async move { client.execute(self.to_request()).await.map_err(Into::into) })
    }
}
