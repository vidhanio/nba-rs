//! a crate for the nba api.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(missing_debug_implementations)]
// #![warn(missing_docs)]

use thiserror::Error;

pub(crate) mod library;
pub(crate) mod stats;

pub use library::NBAHttp;
pub use library::NBAResponse;
pub use stats::NBAStatsHttp;
pub use stats::NBAStatsResponse;

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
