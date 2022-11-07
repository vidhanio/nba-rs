use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Weight {
    #[serde(rename = "LT 200")]
    LessThan200Pounds,

    #[serde(rename = "GT 200")]
    GreaterThan200Pounds,

    #[serde(rename = "LT 225")]
    LessThan225Pounds,

    #[serde(rename = "GT 225")]
    GreaterThan225Pounds,

    #[serde(rename = "LT 250")]
    LessThan250Pounds,

    #[serde(rename = "GT 250")]
    GreaterThan250Pounds,

    #[serde(rename = "LT 275")]
    LessThan275Pounds,

    #[serde(rename = "GT 275")]
    GreaterThan275Pounds,

    #[serde(rename = "LT 300")]
    LessThan300Pounds,

    #[serde(rename = "GT 300")]
    GreaterThan300Pounds,
}
