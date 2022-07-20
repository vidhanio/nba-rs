use std::time::Duration;

use async_trait::async_trait;
use rand::prelude::*;
use reqwest::{
    header::{HeaderMap, REFERER},
    Proxy, StatusCode, Url,
};
use serde_json::Value;

use crate::Result;

pub trait NBAResponse {
    fn new(response: String, status_code: StatusCode, url: Url) -> Self;

    fn response(&self) -> &str;

    /// # Errors
    ///
    /// this will only return errors from [`serde_json::from_str`].
    fn json(&self) -> Result<Value> {
        serde_json::from_str(self.response()).map_err(Into::into)
    }

    fn valid_json(&self) -> bool {
        self.json().is_ok()
    }

    fn url(&self) -> &Url;

    fn status_code(&self) -> StatusCode;
}

#[async_trait]
pub trait NBAHttp {
    type NBAResponse: NBAResponse;

    const BASE_URL: &'static str;

    fn headers() -> HeaderMap;

    fn clean_contents(&self, contents: String) -> String {
        contents
    }

    async fn send_api_request(
        &self,
        endpoint: &str,
        parameters: &[(&str, &str)],
        referer: Option<&str>,
        proxy: Option<&[&str]>,
        headers: Option<HeaderMap>,
        timeout: Option<Duration>,
    ) -> Result<Self::NBAResponse> {
        let base_url = Self::BASE_URL.replace("{endpoint}", endpoint);
        // let endpoint = endpoint.to_lowercase();
        // self.parameters = parameters

        let mut request_headers = headers.unwrap_or_else(|| Self::headers());

        if let Some(referer) = referer {
            request_headers.insert(REFERER, referer.try_into()?);
        }

        let mut parameters = parameters.to_owned();

        parameters.sort_by_key(|&(k, _)| k);

        let request_proxy = proxy.and_then(|ps| ps.choose(&mut thread_rng())).copied();

        let mut builder = reqwest::Client::builder()
            .gzip(true)
            .timeout(timeout.unwrap_or_else(|| Duration::from_secs(5)));

        if let Some(proxy) = request_proxy {
            builder = builder
                .proxy(Proxy::http(proxy)?)
                .proxy(Proxy::https(proxy)?);
        }

        let client = builder.build()?;

        let request = client
            .get(&base_url)
            .query(&parameters)
            .headers(request_headers)
            .build()?;

        let response = client.execute(request).await?;

        let url = response.url().clone();
        let status_code = response.status();
        let contents = response.text().await?;

        Ok(NBAResponse::new(contents, status_code, url))
    }
}
