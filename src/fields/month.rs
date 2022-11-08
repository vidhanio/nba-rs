use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Month {
    #[default]
    AllMonths = 0,

    January = 4,

    February = 5,

    March = 6,

    April = 7,

    May = 8,

    June = 9,

    July = 10,

    August = 11,

    September = 12,

    October = 1,

    November = 2,

    December = 3,
}
