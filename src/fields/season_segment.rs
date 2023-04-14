use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SeasonSegment {
    #[serde(rename = "Pre All-Star")]
    PreAllStar,

    #[serde(rename = "Post All-Star")]
    PostAllStar,
}
