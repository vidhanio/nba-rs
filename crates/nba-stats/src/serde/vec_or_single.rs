use std::{iter, vec};

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

impl<T> IntoIterator for VecOrSingle<T> {
    type IntoIter = VecOrSingleIntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Vec(v) => VecOrSingleIntoIter::Vec(v.into_iter()),
            Self::Single(s) => VecOrSingleIntoIter::Single(iter::once(s)),
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

#[derive(Clone, Debug)]
pub enum VecOrSingleIntoIter<T> {
    Vec(vec::IntoIter<T>),
    Single(iter::Once<T>),
}

impl<T> Iterator for VecOrSingleIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Vec(v) => v.next(),
            Self::Single(s) => s.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::Vec(v) => v.size_hint(),
            Self::Single(s) => s.size_hint(),
        }
    }
}
