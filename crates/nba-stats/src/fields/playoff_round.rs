use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PlayoffRound {
    #[default]
    AllPlayoffRounds = 0,

    ConferenceQuarterFinals = 1,

    ConferenceSemiFinals = 2,

    ConferenceFinals = 3,

    Finals = 4,
}
