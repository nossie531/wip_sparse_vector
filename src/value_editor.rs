//! Provider of [`ValueEditor`].

use crate::prelude::*;
use std::fmt::Debug;
use std::mem;
use std::ops::{Deref, DerefMut};

/// Editor on sparse vector value.
#[must_use]
#[derive(Debug)]
pub struct ValueEditor<'a, T>
where
    T: PartialEq,
{
    /// Underlyiing vec.
    vec: &'a mut SparseVec<T>,

    /// Target index.
    index: usize,

    /// Edited new value.
    new_value: Option<T>,
}

impl<'a, T> ValueEditor<'a, T>
where
    T: PartialEq,
{
    /// Creates a new instance.
    pub(crate) fn new(vec: &'a mut SparseVec<T>, index: usize) -> Self {
        Self {
            vec,
            index,
            new_value: None,
        }
    }
}

impl<'a, T> Deref for ValueEditor<'a, T>
where
    T: PartialEq,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let new_value = self.new_value.as_ref();
        match new_value {
            Some(x) => x,
            None => &self.vec[self.index],
        }
    }
}

impl<'a, T> DerefMut for ValueEditor<'a, T>
where
    T: PartialEq,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.new_value.is_none() {
            let padding = self.vec.padding_val();
            let map_value = self.vec.map.get_mut(&self.index);
            self.new_value = Some(match map_value {
                None => padding,
                Some(x) => mem::replace(x, padding),
            });
        }

        self.new_value.as_mut().unwrap()
    }
}

impl<'a, T> Drop for ValueEditor<'a, T>
where
    T: PartialEq,
{
    fn drop(&mut self) {
        if self.new_value.is_none() {
            return;
        }

        let new_value = self.new_value.take().unwrap();
        if &new_value == self.vec.padding_ref() {
            self.vec.map.remove(&self.index);
        } else {
            self.vec.map.insert(self.index, new_value);
        }
    }
}
