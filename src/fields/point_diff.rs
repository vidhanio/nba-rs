use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PointDiff {
    #[default]
    LessThanOrEqualTo5 = 5,

    LessThanOrEqualTo4 = 4,

    LessThanOrEqualTo3 = 3,

    LessThanOrEqualTo2 = 2,

    LessThanOrEqualTo1 = 1,
}
