use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerMode {
    #[serde(rename = "Totals")]
    Totals,

    #[default]
    #[serde(rename = "PerGame")]
    PerGame,

    #[serde(rename = "Per100Possessions")]
    Per100Poss,

    #[serde(rename = "Per100Plays")]
    Per100Plays,

    #[serde(rename = "Per48")]
    Per48Minutes,

    #[serde(rename = "Per40")]
    Per40Minutes,

    #[serde(rename = "Per36")]
    Per36Minutes,

    #[serde(rename = "PerMinute")]
    Per1Minute,

    #[serde(rename = "PerPossession")]
    Per1Poss,

    #[serde(rename = "PerPlay")]
    Per1Play,

    #[serde(rename = "MinutesPer")]
    MinutesPer,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerMode48 {
    #[serde(rename = "Totals")]
    Totals,

    #[default]
    #[serde(rename = "PerGame")]
    PerGame,

    #[serde(rename = "Per48")]
    Per48Minutes,
}

super::convert_subset! {
    PerMode48 => PerMode {
        Totals,
        PerGame,
        Per48Minutes,
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerModeGame {
    #[serde(rename = "Totals")]
    Totals,

    #[default]
    #[serde(rename = "PerGame")]
    PerGame,
}

super::convert_subset! {
    PerModeGame => PerMode {
        Totals,
        PerGame,
    }
}

super::convert_subset! {
    PerModeGame => PerMode48 {
        Totals,
        PerGame,
    }
}
