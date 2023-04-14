use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Height {
    #[serde(rename = "LT 6-0")]
    LessThan6Feet0Inches,

    #[serde(rename = "GT 6-0")]
    GreaterThan6Feet0Inches,

    #[serde(rename = "LT 6-4")]
    LessThan6Feet4Inches,

    #[serde(rename = "GT 6-4")]
    GreaterThan6Feet4Inches,

    #[serde(rename = "LT 6-7")]
    LessThan6Feet7Inches,

    #[serde(rename = "GT 6-7")]
    GreaterThan6Feet7Inches,

    #[serde(rename = "LT 6-10")]
    LessThan6Feet10Inches,

    #[serde(rename = "GT 6-10")]
    GreaterThan6Feet10Inches,

    #[serde(rename = "LT 7-0")]
    LessThan7Feet0Inches,

    #[serde(rename = "GT 7-0")]
    GreaterThan7Feet0Inches,
}
