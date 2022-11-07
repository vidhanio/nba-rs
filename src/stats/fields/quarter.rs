use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Quarter {
    #[default]
    AllQuarters = 0,

    FirstQuarter = 1,

    SecondQuarter = 2,

    ThirdQuarter = 3,

    FourthQuarter = 4,

    Overtime1 = 5,

    Overtime2 = 6,

    Overtime3 = 7,

    Overtime4 = 8,

    Overtime5 = 9,

    Overtime6 = 10,

    Overtime7 = 11,

    Overtime8 = 12,

    Overtime9 = 13,

    Overtime10 = 14,
}
