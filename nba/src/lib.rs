//! a crate for the nba api.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(missing_debug_implementations)]
// #![warn(missing_docs)]

use reqwest::{
    header::{HeaderValue, REFERER},
    Client, Response,
};
use thiserror::Error;

pub(crate) mod endpoints;

/// An error which encompasses all possible errors which may occur when using the nba api.
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
    pub async fn send_request<'a, I, S, R>(&self, endpoint: &str, properties: I) -> Result<Response>
    where
        I: IntoIterator<Item = &'a (S, R)> + Send + Sync,
        S: AsRef<str> + 'a,
        R: AsRef<str> + 'a,
    {
        let base_url = format!("https://stats.nba.com/stats/{endpoint}");

        let mut properties = properties
            .into_iter()
            .map(|(k, v)| (k.as_ref().to_owned(), v.as_ref().to_owned()))
            .collect::<Vec<_>>();

        properties.sort_unstable_by(|(k1, _), (k2, _)| k1.cmp(k2));

        let request = self
            .client
            .get(&base_url)
            .query(&properties)
            .headers(
                [(REFERER, "https://stats.nba.com/")]
                    .into_iter()
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
