#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

pub mod analyzers;
pub mod consts;

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[macro_export]
macro_rules! regex {
    ($($re:literal),+ $(,)?) => {($({
        static RE: ::once_cell::sync::OnceCell<::regex::Regex> = ::once_cell::sync::OnceCell::new();
        RE.get_or_init(|| ::regex::Regex::new($re).unwrap())
    }),+)};
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
#[serde(rename_all = "snake_case")]
pub enum Analysis {
    Valid(ValidAnalysis),
    Invalid(InvalidAnalysis),
    Deprecated(DeprecatedAnalysis),
    Unknown(UnknownAnalysis),
}

impl Analysis {
    #[must_use]
    pub fn map_valid<F>(self, f: F) -> Self
    where
        F: FnOnce(ValidAnalysis) -> Self,
    {
        match self {
            Self::Valid(analysis) => f(analysis),
            other => other,
        }
    }

    #[must_use]
    pub fn into_valid(self) -> Self {
        Self::Valid(self.into())
    }

    #[must_use]
    pub fn into_invalid(self) -> Self {
        Self::Invalid(self.into())
    }

    #[must_use]
    pub fn into_deprecated(self) -> Self {
        Self::Deprecated(self.into())
    }

    #[must_use]
    pub fn into_unknown(self) -> Self {
        Self::Unknown(self.into())
    }

    #[must_use]
    pub fn endpoint(&self) -> &str {
        match self {
            Self::Valid(ValidAnalysis { endpoint, .. })
            | Self::Invalid(InvalidAnalysis { endpoint, .. })
            | Self::Deprecated(DeprecatedAnalysis { endpoint, .. })
            | Self::Unknown(UnknownAnalysis { endpoint, .. }) => endpoint,
        }
    }

    #[must_use]
    pub fn response(&self) -> Option<&str> {
        match self {
            Self::Valid(ValidAnalysis { response, .. })
            | Self::Invalid(InvalidAnalysis { response, .. })
            | Self::Unknown(UnknownAnalysis { response, .. }) => response.as_deref(),
            Self::Deprecated(DeprecatedAnalysis { .. }) => None,
        }
    }
}

impl From<ValidAnalysis> for Analysis {
    fn from(analysis: ValidAnalysis) -> Self {
        Self::Valid(analysis)
    }
}

impl From<InvalidAnalysis> for Analysis {
    fn from(analysis: InvalidAnalysis) -> Self {
        Self::Invalid(analysis)
    }
}

impl From<DeprecatedAnalysis> for Analysis {
    fn from(analysis: DeprecatedAnalysis) -> Self {
        Self::Deprecated(analysis)
    }
}

impl From<UnknownAnalysis> for Analysis {
    fn from(analysis: UnknownAnalysis) -> Self {
        Self::Unknown(analysis)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidAnalysis {
    pub endpoint: String,
    pub response: Option<String>,
    pub data_sets: Option<HashMap<String, HashSet<String>>>,
    pub parameters: Option<HashMap<String, Parameter>>,
}

impl ValidAnalysis {
    #[must_use]
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_owned(),
            response: None,
            data_sets: None,
            parameters: None,
        }
    }

    #[must_use]
    pub fn with_response(self, response: &str) -> Self {
        Self {
            response: Some(response.to_owned()),
            ..self
        }
    }

    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn with_data_sets(self, data_sets: HashMap<String, HashSet<String>>) -> Self {
        Self {
            data_sets: Some(data_sets),
            ..self
        }
    }

    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn with_parameters(self, parameters: HashMap<String, Parameter>) -> Self {
        Self {
            parameters: Some(parameters),
            ..self
        }
    }
}

impl From<Analysis> for ValidAnalysis {
    fn from(analysis: Analysis) -> Self {
        match analysis {
            Analysis::Valid(analysis) => analysis,
            Analysis::Invalid(analysis) => analysis.into(),
            Analysis::Deprecated(analysis) => analysis.into(),
            Analysis::Unknown(analysis) => analysis.into(),
        }
    }
}

impl From<InvalidAnalysis> for ValidAnalysis {
    fn from(
        InvalidAnalysis {
            endpoint,
            response,
            data_sets,
            parameters,
        }: InvalidAnalysis,
    ) -> Self {
        Self {
            endpoint,
            response,
            data_sets,
            parameters,
        }
    }
}

impl From<DeprecatedAnalysis> for ValidAnalysis {
    fn from(DeprecatedAnalysis { endpoint, .. }: DeprecatedAnalysis) -> Self {
        Self {
            endpoint,
            response: None,
            data_sets: None,
            parameters: None,
        }
    }
}

impl From<UnknownAnalysis> for ValidAnalysis {
    fn from(
        UnknownAnalysis {
            endpoint, response, ..
        }: UnknownAnalysis,
    ) -> Self {
        Self {
            endpoint,
            response,
            data_sets: None,
            parameters: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvalidAnalysis {
    pub endpoint: String,
    pub response: Option<String>,
    pub data_sets: Option<HashMap<String, HashSet<String>>>,
    pub parameters: Option<HashMap<String, Parameter>>,
}

impl InvalidAnalysis {
    #[must_use]
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_owned(),
            response: None,
            data_sets: None,
            parameters: None,
        }
    }

    #[must_use]
    pub fn with_response(self, response: &str) -> Self {
        Self {
            response: Some(response.to_owned()),
            ..self
        }
    }

    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn with_data_sets(self, data_sets: HashMap<String, HashSet<String>>) -> Self {
        Self {
            data_sets: Some(data_sets),
            ..self
        }
    }

    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn with_parameters(self, parameters: HashMap<String, Parameter>) -> Self {
        Self {
            parameters: Some(parameters),
            ..self
        }
    }
}

impl From<Analysis> for InvalidAnalysis {
    fn from(analysis: Analysis) -> Self {
        match analysis {
            Analysis::Valid(analysis) => analysis.into(),
            Analysis::Invalid(analysis) => analysis,
            Analysis::Deprecated(analysis) => analysis.into(),
            Analysis::Unknown(analysis) => analysis.into(),
        }
    }
}

impl From<ValidAnalysis> for InvalidAnalysis {
    fn from(
        ValidAnalysis {
            endpoint,
            response,
            data_sets,
            parameters,
        }: ValidAnalysis,
    ) -> Self {
        Self {
            endpoint,
            response,
            data_sets,
            parameters,
        }
    }
}

impl From<DeprecatedAnalysis> for InvalidAnalysis {
    fn from(DeprecatedAnalysis { endpoint, .. }: DeprecatedAnalysis) -> Self {
        Self {
            endpoint,
            response: None,
            data_sets: None,
            parameters: None,
        }
    }
}

impl From<UnknownAnalysis> for InvalidAnalysis {
    fn from(
        UnknownAnalysis {
            endpoint, response, ..
        }: UnknownAnalysis,
    ) -> Self {
        Self {
            endpoint,
            response,
            data_sets: None,
            parameters: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeprecatedAnalysis {
    pub endpoint: String,
}

impl From<Analysis> for DeprecatedAnalysis {
    fn from(analysis: Analysis) -> Self {
        match analysis {
            Analysis::Valid(analysis) => analysis.into(),
            Analysis::Invalid(analysis) => analysis.into(),
            Analysis::Deprecated(analysis) => analysis,
            Analysis::Unknown(analysis) => analysis.into(),
        }
    }
}

impl DeprecatedAnalysis {
    #[must_use]
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_owned(),
        }
    }
}

impl From<ValidAnalysis> for DeprecatedAnalysis {
    fn from(ValidAnalysis { endpoint, .. }: ValidAnalysis) -> Self {
        Self { endpoint }
    }
}

impl From<InvalidAnalysis> for DeprecatedAnalysis {
    fn from(InvalidAnalysis { endpoint, .. }: InvalidAnalysis) -> Self {
        Self { endpoint }
    }
}

impl From<UnknownAnalysis> for DeprecatedAnalysis {
    fn from(UnknownAnalysis { endpoint, .. }: UnknownAnalysis) -> Self {
        Self { endpoint }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnknownAnalysis {
    pub endpoint: String,
    pub response: Option<String>,
}

impl From<Analysis> for UnknownAnalysis {
    fn from(analysis: Analysis) -> Self {
        match analysis {
            Analysis::Valid(analysis) => analysis.into(),
            Analysis::Invalid(analysis) => analysis.into(),
            Analysis::Deprecated(analysis) => analysis.into(),
            Analysis::Unknown(analysis) => analysis,
        }
    }
}

impl UnknownAnalysis {
    #[must_use]
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_owned(),
            response: None,
        }
    }

    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn with_response(self, response: &str) -> Self {
        Self {
            response: Some(response.to_owned()),
            ..self
        }
    }
}

impl From<ValidAnalysis> for UnknownAnalysis {
    fn from(
        ValidAnalysis {
            endpoint, response, ..
        }: ValidAnalysis,
    ) -> Self {
        Self { endpoint, response }
    }
}

impl From<InvalidAnalysis> for UnknownAnalysis {
    fn from(
        InvalidAnalysis {
            endpoint, response, ..
        }: InvalidAnalysis,
    ) -> Self {
        Self { endpoint, response }
    }
}

impl From<DeprecatedAnalysis> for UnknownAnalysis {
    fn from(DeprecatedAnalysis { endpoint, .. }: DeprecatedAnalysis) -> Self {
        Self {
            endpoint,
            response: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Parameter {
    pub required: Option<bool>,
    pub nullable: Option<bool>,
    pub pattern: Option<String>,
    #[serde(rename = "type")]
    pub ty: Option<Type>,
}

impl Parameter {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            required: None,
            nullable: None,
            pattern: None,
            ty: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    String,
    Number,
    Bool,
    Object,
    Array,
    Null,
}

impl From<Value> for Type {
    fn from(value: Value) -> Self {
        match value {
            Value::String(_) => Self::String,
            Value::Number(_) => Self::Number,
            Value::Bool(_) => Self::Bool,
            Value::Object(_) => Self::Object,
            Value::Array(_) => Self::Array,
            Value::Null => Self::Null,
        }
    }
}
