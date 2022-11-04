//! The NBA Stats API.

#![allow(missing_docs)]

use std::{future::Future, pin::Pin};

use reqwest::{Client, Request};
use serde::{de::DeserializeOwned, Serialize};

use crate::{BasicResponse, Response, Result, CLIENT};

pub mod alltimeleadersgrids;
pub mod assistleaders;
pub mod response;

pub use alltimeleadersgrids::AllTimeLeadersGrids;
pub use assistleaders::AssistLeaders;

type BoxedResultFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T>> + Send + 'a>>;

/// A trait which represents an endpoint in the NBA Stats API.
pub trait Endpoint: Serialize + DeserializeOwned {
    /// The endpoint's path.
    const ENDPOINT: &'static str;

    /// The type of the result sets returned by the endpoint.
    type ResultSets: DeserializeOwned;

    /// Creates a [`reqwest::Request`] from `self` using the provided
    /// [`reqwest::Client`].
    fn to_request_with_client(&self, client: &Client) -> Request {
        client
            .get(format!("https://stats.nba.com/stats/{}", Self::ENDPOINT))
            .query(&self)
            .build()
            .expect("request should build")
    }

    /// Creates a [`reqwest::Request`] from `self` using [`crate::CLIENT`].
    fn to_request(&self) -> Request {
        self.to_request_with_client(&CLIENT)
    }

    /// Sends a request to the endpoint and returns the response deserialized to
    /// [`Response<Self, Self::ResultSets>`] using [`crate::CLIENT`].
    fn send<'s, 'f>(&'s self) -> BoxedResultFuture<'f, Response<Self, Self::ResultSets>>
    where
        's: 'f,
        Self: Sync,
    {
        Box::pin(async move { self.send_with_client(&CLIENT).await })
    }

    /// Sends a request to the endpoint and returns the response deserialized to
    /// [`Response<Self, Self::ResultSets>`] using the provided
    /// [`reqwest::Client`].
    fn send_with_client<'s, 'c, 'f>(
        &'s self,
        client: &'c Client,
    ) -> BoxedResultFuture<'f, Response<Self, Self::ResultSets>>
    where
        's: 'f,
        'c: 'f,
        Self: Sync,
    {
        Box::pin(async move {
            self.send_raw_with_client(client)
                .await?
                .json()
                .await
                .map_err(Into::into)
        })
    }

    /// Sends a request to the endpoint and returns the response deserialized to
    /// [`BasicResponse`] using [`crate::CLIENT`].
    fn send_basic<'s, 'f>(&'s self) -> BoxedResultFuture<'f, BasicResponse>
    where
        's: 'f,
        Self: Sync,
    {
        Box::pin(async move { self.send_basic_with_client(&CLIENT).await })
    }

    /// Sends a request to the endpoint and returns the response deserialized to
    /// [`BasicResponse`] using the provided [`reqwest::Client`].
    fn send_basic_with_client<'s, 'c, 'f>(
        &'s self,
        client: &'c Client,
    ) -> BoxedResultFuture<'f, BasicResponse>
    where
        's: 'f,
        'c: 'f,
        Self: Sync,
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
        Self: Sync,
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
        Self: Sync,
    {
        Box::pin(async move { client.execute(self.to_request()).await.map_err(Into::into) })
    }
}
