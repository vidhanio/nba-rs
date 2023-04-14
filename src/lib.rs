//! A crate for the NBA Stats API.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![allow(clippy::module_name_repetitions)]
#![warn(missing_docs)]

use once_cell::sync::Lazy;
use reqwest::{
    header::{HeaderValue, REFERER},
    Client, ClientBuilder,
};
use stats::endpoint::macros::endpoint;
use thiserror::Error;

pub mod fields;
pub mod live;
mod serde;
pub mod stats;

pub use self::stats::{
    endpoint::Endpoint,
    response::{
        basic::{BasicResponse, BasicResultSet},
        Response,
    },
};

/// The [`reqwest::ClientBuilder`] used in [`CLIENT`].
///
/// This builder is configured to use the NBA Stats API's referer by default.
pub fn client_builder() -> ClientBuilder {
    let headers = std::iter::once((REFERER, "https://www.nba.com/"))
        .map(|(name, value)| (name, HeaderValue::from_static(value)))
        .collect();

    Client::builder().default_headers(headers)
}

/// The default [`reqwest::Client`] used by [`Endpoint`]s.
///
/// This client is configured to use the NBA Stats API's referer by default.
pub static CLIENT: Lazy<Client> = Lazy::new(|| client_builder().build().unwrap());

/// An error which encompasses all possible errors which may occur when using
/// this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// A [`reqwest::Error`] occurred.
    #[error("a reqwest error occurred")]
    Reqwest(#[from] reqwest::Error),

    /// A [`serde_json::Error`] occurred.
    #[error("a serde_json error occurred")]
    Serde(#[from] serde_json::Error),
}

/// A convenience [`std::result::Result`] which uses a [`enum@crate::Error`] as
/// its error type.
pub type Result<T, E = Error> = std::result::Result<T, E>;
