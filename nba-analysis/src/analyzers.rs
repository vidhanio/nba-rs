pub mod params;
mod utils;

use futures::prelude::*;
use log::info;
use std::{collections::HashMap, path::Path, pin::Pin};
use tokio::fs;

use crate::{Analysis, UnknownAnalysis};

pub enum DataSource<'a> {
    EndpointList(&'a [&'a str]),
    JsonFilepath(&'a Path),
}

pub async fn deserialize_from(
    data: DataSource<'_>,
) -> impl TryStream<Ok = Analysis, Error = anyhow::Error> + '_ {
    match data {
        DataSource::EndpointList(endpoints) => {
            Box::pin(stream::iter(endpoints.iter().copied()).map(Ok).and_then(
                |endpoint| async move {
                    info!("getting endpoint: {endpoint}");

                    utils::get_response_text::<&str, &str>(endpoint, &[])
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
            .map_or_else::<Pin<Box<dyn Stream<Item = _>>>, _, _>(
                |e| Box::pin(stream::once(async { Err(e) })),
                |a| Box::pin(stream::iter(a.into_values()).map(Ok)),
            ),
    }
}

#[must_use]
pub fn deprecation(analysis: Analysis) -> Analysis {
    if utils::is_deprecated(analysis.response().unwrap_or_default()) {
        Analysis::Deprecated(analysis.into())
    } else {
        analysis
    }
}
