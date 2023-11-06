use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LastNGames {
    #[default]
    AllGames = 0,

    LastGame = 1,

    Last2Games = 2,

    Last3Games = 3,

    Last4Games = 4,

    Last5Games = 5,

    Last6Games = 6,

    Last7Games = 7,

    Last8Games = 8,

    Last9Games = 9,

    Last10Games = 10,

    Last11Games = 11,

    Last12Games = 12,

    Last13Games = 13,

    Last14Games = 14,

    Last15Games = 15,
}
