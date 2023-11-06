use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// The type of period.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PeriodType {
    /// A regular period.
    Regular,

    /// An overtime period.
    Overtime,
}

impl PeriodType {
    /// Returns whether the period type is regular.
    #[must_use]
    pub const fn is_regular(self) -> bool {
        matches!(self, Self::Regular)
    }

    /// Returns whether the period type is overtime.
    #[must_use]
    pub const fn is_overtime(self) -> bool {
        matches!(self, Self::Overtime)
    }
}

impl Display for PeriodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Regular => write!(f, "REGULAR"),
            Self::Overtime => write!(f, "OVERTIME"),
        }
    }
}
