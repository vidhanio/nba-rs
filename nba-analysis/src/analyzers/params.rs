use std::collections::HashMap;

use log::info;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{regex, Analysis, ValidAnalysis};

#[must_use]
pub fn from_json(analysis: Analysis) -> Analysis {
    analysis.map_valid(|analysis| {
        #[derive(Deserialize, Serialize)]
        struct Response {
            parameters: HashMap<String, Value>,
        }

        let response = match &analysis.response {
            Some(response) => response,
            None => return Analysis::Valid(analysis),
        };

        let new_params = match serde_json::from_str::<Response>(response).map(|r| r.parameters) {
            Ok(p) => p.into_iter(),
            Err(_) => {
                return Analysis::Valid(analysis);
            }
        };

        let mut parameters = analysis.parameters.unwrap_or_default();

        new_params.for_each(|(k, v)| {
            parameters.entry(k).or_default().required = Some(!v.is_null());
        });

        Analysis::Valid(ValidAnalysis {
            parameters: Some(parameters),
            ..analysis
        })
    })
}

#[must_use]
pub fn required(analysis: Analysis) -> Analysis {
    analysis.map_valid(|analysis| {
        let re = regex!(r"(?P<param>[\w]+)(?: property)? is required");

        let response = analysis.response.unwrap_or_default();

        let required_params = re.captures_iter(&response).map(|c| {
            c.name("param")
                .expect("param should exist in match")
                .as_str()
        });

        let mut parameters = analysis.parameters.unwrap_or_default();

        required_params.for_each(|param| {
            parameters.entry(param.to_owned()).or_default().required = Some(true);
        });

        Analysis::Valid(ValidAnalysis {
            response: Some(response),
            parameters: Some(parameters),
            ..analysis
        })
    })
}
