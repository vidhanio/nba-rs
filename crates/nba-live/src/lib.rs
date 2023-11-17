//! The NBA live API.

mod meta;
pub mod playbyplay;
mod serde;

use thiserror::Error;

pub use self::meta::Meta;

/// A general-purpose error type encompassing all possible errors that may occur
/// in this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// A [`reqwest::Error`].
    #[error("reqwest error")]
    Reqwest(#[from] reqwest::Error),

    /// A [`serde_json::Error`].
    #[error("serde error")]
    Serde(#[from] serde_json::Error),
}

type Result<T> = std::result::Result<T, Error>;
