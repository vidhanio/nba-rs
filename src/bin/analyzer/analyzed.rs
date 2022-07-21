use std::collections::{HashMap, HashSet};

use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct AnalyzedEndpoint {
    endpoint: String,
    last_validated_date: String,
    status: Status,
    data_sets: HashMap<String, HashSet<String>>,
    parameters: HashMap<String, Parameter>,
}

#[derive(Serialize, Deserialize)]
struct Parameter {
    required: bool,
    nullable: bool,
    #[serde(with = "serde_regex")]
    pattern: Option<Regex>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Status {
    Success,
    Invalid,
    Deprecated,
}
