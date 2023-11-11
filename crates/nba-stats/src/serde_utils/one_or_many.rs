use std::{iter, vec};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

impl<T> OneOrMany<T> {
    pub fn into_vec(self) -> Vec<T> {
        match self {
            Self::Many(v) => v,
            Self::One(s) => vec![s],
        }
    }

    pub fn iter(&self) -> OneOrManyIter<'_, T> {
        match self {
            Self::Many(v) => OneOrManyIter::Many(v.iter()),
            Self::One(s) => OneOrManyIter::One(iter::once(s)),
        }
    }
}

impl<T> IntoIterator for OneOrMany<T> {
    type IntoIter = OneOrManyIntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Many(v) => OneOrManyIntoIter::Many(v.into_iter()),
            Self::One(s) => OneOrManyIntoIter::One(iter::once(s)),
        }
    }
}

impl<'a, T> IntoIterator for &'a OneOrMany<T> {
    type IntoIter = OneOrManyIter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> From<Vec<T>> for OneOrMany<T> {
    fn from(value: Vec<T>) -> Self {
        Self::Many(value)
    }
}

impl<T> From<T> for OneOrMany<T> {
    fn from(value: T) -> Self {
        Self::One(value)
    }
}

impl<T> From<OneOrMany<T>> for Vec<T> {
    fn from(value: OneOrMany<T>) -> Self {
        match value {
            OneOrMany::Many(v) => v,
            OneOrMany::One(s) => vec![s],
        }
    }
}

#[derive(Clone, Debug)]
pub enum OneOrManyIntoIter<T> {
    One(iter::Once<T>),
    Many(vec::IntoIter<T>),
}

impl<T> Iterator for OneOrManyIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Many(v) => v.next(),
            Self::One(s) => s.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::Many(v) => v.size_hint(),
            Self::One(s) => s.size_hint(),
        }
    }
}

impl<T> ExactSizeIterator for OneOrManyIntoIter<T> {
    fn len(&self) -> usize {
        match self {
            Self::Many(v) => v.len(),
            Self::One(s) => s.len(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum OneOrManyIter<'a, T> {
    One(iter::Once<&'a T>),
    Many(std::slice::Iter<'a, T>),
}

impl<'a, T> Iterator for OneOrManyIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Many(v) => v.next(),
            Self::One(s) => s.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::Many(v) => v.size_hint(),
            Self::One(s) => s.size_hint(),
        }
    }
}

impl<'a, T> ExactSizeIterator for OneOrManyIter<'a, T> {
    fn len(&self) -> usize {
        match self {
            Self::Many(v) => v.len(),
            Self::One(s) => s.len(),
        }
    }
}
