//! a crate for the nba api.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
// #![warn(missing_docs)]

use reqwest::{
    header::{HeaderValue, REFERER},
    Client, Response,
};
use thiserror::Error;

pub(crate) mod endpoints;
pub mod parameters;

/// An error which encompasses all possible errors which may occur when using
/// the nba api.
#[derive(Debug, Error)]
pub enum Error {
    /// A variant which contains a [`reqwest::Error`]
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("{0}")]
    /// A variant which contains a [`serde_json::Error`]
    Serde(#[from] serde_json::Error),

    /// A variant which contains a [`reqwest::header::InvalidHeaderValue`]
    #[error("{0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
}

/// A result alias which may be useful when working with this crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct NbaClient {
    client: Client,
}

impl NbaClient {
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// sends a request to the nba api and returns the response.
    ///
    /// # Errors
    ///
    /// this function will error if it fails to send the request.
    pub async fn send_request<'a, I>(&self, endpoint: &str, parameters: I) -> Result<Response>
    where
        I: IntoIterator<Item = (&'a str, &'a str)> + Send + Sync,
    {
        let base_url = format!("https://stats.nba.com/stats/{endpoint}");

        let mut parameters = parameters.into_iter().collect::<Vec<_>>();

        parameters.sort_unstable_by(|(k1, _), (k2, _)| k1.cmp(k2));

        let request = self
            .client
            .get(&base_url)
            .query(&parameters)
            .headers(
                std::iter::once((REFERER, "https://stats.nba.com/"))
                    .map(|(k, v)| {
                        (
                            k,
                            v.parse::<HeaderValue>()
                                .expect("header value should be valid"),
                        )
                    })
                    .collect(),
            )
            .build()?;

        self.client.execute(request).await.map_err(Into::into)
    }
}

impl Default for NbaClient {
    fn default() -> Self {
        Self::new()
    }
}
