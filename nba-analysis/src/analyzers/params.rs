use log::info;

use crate::{regex, Analysis, ValidAnalysis};

#[must_use]
pub fn required(analysis: Analysis) -> Analysis {
    match analysis {
        Analysis::Valid(analysis) => {
            info!(
                "updating required params for endpoint: {}",
                analysis.endpoint
            );

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
        }
        analysis => analysis,
    }
}
