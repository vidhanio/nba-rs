use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum PerModeWith48 {
    #[default]
    #[serde(rename = "Totals")]
    Totals,

    #[serde(rename = "PerGame")]
    PerGame,

    #[serde(rename = "Per48")]
    Per48,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PerModeStat {
    Totals(u32),

    PerGame(f64),
}

impl PerModeStat {
    #[must_use]
    pub fn as_f64(self) -> f64 {
        match self {
            Self::Totals(v) => v.into(),
            Self::PerGame(v) => v,
        }
    }
}

impl Default for PerModeStat {
    fn default() -> Self {
        Self::Totals(0)
    }
}
