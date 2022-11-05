use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VecOrSingle<T> {
    Vec(Vec<T>),
    Single(T),
}

impl<T> VecOrSingle<T> {
    pub fn into_vec(self) -> Vec<T> {
        match self {
            Self::Vec(v) => v,
            Self::Single(s) => vec![s],
        }
    }
}

impl<T> From<Vec<T>> for VecOrSingle<T> {
    fn from(value: Vec<T>) -> Self {
        Self::Vec(value)
    }
}

impl<T> From<T> for VecOrSingle<T> {
    fn from(value: T) -> Self {
        Self::Single(value)
    }
}

impl<T> From<VecOrSingle<T>> for Vec<T> {
    fn from(value: VecOrSingle<T>) -> Self {
        match value {
            VecOrSingle::Vec(v) => v,
            VecOrSingle::Single(s) => vec![s],
        }
    }
}
