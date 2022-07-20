pub mod endpoints;
pub mod parameters;

use crate::{library::NBAResponse, NBAHttp, Result};
use reqwest::{
    header::{HeaderMap, ACCEPT, ACCEPT_ENCODING, CONNECTION, HOST, REFERER, USER_AGENT},
    StatusCode, Url,
};

#[derive(Debug, Clone)]
pub struct NBAStatsResponse {
    response: String,
    status_code: StatusCode,
    url: Url,
}

impl NBAStatsResponse {}

impl NBAResponse for NBAStatsResponse {
    fn new(response: String, status_code: StatusCode, url: Url) -> Self {
        Self {
            response,
            status_code,
            url,
        }
    }

    fn response(&self) -> &str {
        &self.response
    }

    fn url(&self) -> &Url {
        &self.url
    }

    fn status_code(&self) -> StatusCode {
        self.status_code
    }
}

#[derive(Debug, Clone)]
pub struct NBAStatsHttp;

impl NBAHttp for NBAStatsHttp {
    type NBAResponse = NBAStatsResponse;

    const BASE_URL: &'static str = "https://stats.nba.com/stats/{endpoint}";

    fn headers() -> HeaderMap {
        [(REFERER, "https://stats.nba.com/")].into_iter().fold(
            HeaderMap::new(),
            |mut headers, (key, value)| {
                headers.insert(key, value.parse().unwrap());
                headers
            },
        )
    }
}
