pub mod params;
mod utils;

use futures::{prelude::*, stream::BoxStream};
use log::info;
use std::{collections::HashMap, path::Path};
use tokio::fs;

use crate::{Analysis, UnknownAnalysis};

pub enum DataSource<'a> {
    EndpointList(&'a [&'a str]),
    JsonFilepath(&'a Path),
}

trait X {
    type T;
}

pub async fn deserialize_from(
    data: DataSource<'_>,
) -> impl TryStream<Ok = Analysis, Error = anyhow::Error> + '_ {
    match data {
        DataSource::EndpointList(endpoints) => {
            Box::pin(stream::iter(endpoints.iter().copied()).map(Ok).and_then(
                |endpoint| async move {
                    info!("getting endpoint from api: {endpoint}");

                    utils::get_response_text(endpoint, [])
                        .await
                        .map(|response| {
                            Analysis::Unknown(UnknownAnalysis {
                                endpoint: endpoint.to_owned(),
                                response: Some(response),
                            })
                        })
                },
            ))
        }
        DataSource::JsonFilepath(file) => fs::read_to_string(file)
            .await
            .map_err(anyhow::Error::from)
            .and_then(|analysis| {
                serde_json::from_str::<HashMap<String, Analysis>>(&analysis).map_err(Into::into)
            })
            .map_or_else::<BoxStream<_>, _, _>(
                |e| Box::pin(stream::once(async { Err(e) })),
                |a| {
                    Box::pin(stream::iter(a.into_values()).map(|a| {
                        info!("getting endpoint from json: {}", a.endpoint());

                        Ok(a)
                    }))
                },
            ),
    }
}

#[must_use]
pub fn deprecation(analysis: Analysis) -> Analysis {
    let response = match analysis.response() {
        Some(response) => response,
        None => return analysis.into_deprecated(),
    };

    if utils::is_deprecated(response) {
        analysis.into_deprecated()
    } else {
        analysis.into_valid()
    }
}
