use std::time::Duration;

use anyhow::Result;
use tokio::time;

use crate::consts::CLIENT;

/// # Errors
///
/// This function will return an error if the request fails.
pub async fn get_response_text<S, R>(endpoint: &str, params: &[(S, R)]) -> Result<String>
where
    S: AsRef<str> + Send + Sync,
    R: AsRef<str> + Send + Sync,
{
    time::sleep(Duration::from_secs(1)).await;

    CLIENT
        .send_request(&endpoint.to_ascii_lowercase(), params.iter())
        .await?
        .text()
        .await
        .map_err(Into::into)
}

#[must_use]
pub fn is_deprecated(response: &str) -> bool {
    response.starts_with(r"<!DOCTYPE html>")
}
