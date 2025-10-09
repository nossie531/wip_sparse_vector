use crate::util;
use only_one::prelude::*;
use pstd::collections::btree_map::Entry;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::ops::{Deref, DerefMut};

/// Editor on sparse vector value.
pub struct ValueEditor<'a, T>
where
    T: PartialEq,
{
    padding: &'a T,
    entry: One<Entry<'a, usize, T>>,
    new_value: Option<T>,
}

impl<'a, T> ValueEditor<'a, T>
where
    T: PartialEq,
{
    pub fn set(&mut self, value: T) -> &mut Self {
        self.new_value = Some(value);
        self
    }

    pub(crate) fn new(padding: &'a T, entry: Entry<'a, usize, T>) -> Self {
        Self {
            padding: padding,
            entry: One::new(entry),
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
        let entry = &*self.entry;
        match (new_value, entry) {
            (Some(x), _) => x,
            (None, Entry::Occupied(e)) => e.get(),
            (None, Entry::Vacant(_)) => self.padding,
        }
    }
}

impl<'a, T> DerefMut for ValueEditor<'a, T>
where
    T: Clone + PartialEq,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.new_value.is_none() {
            self.new_value = Some(
                match &*self.entry {
                    Entry::Vacant(_) => self.padding,
                    Entry::Occupied(x) => x.get(),
                }
                .clone(),
            );
        }

        self.new_value.as_mut().unwrap()
    }
}

impl<'a, T> Drop for ValueEditor<'a, T>
where
    T: PartialEq,
{
    fn drop(&mut self) {
        let new_value = self.new_value.take();
        let entry = One::take(&mut self.entry);
        match (new_value, entry) {
            (None, _) => {
                return;
            }
            (Some(v), Entry::Vacant(e)) => {
                if v != *self.padding {
                    e.insert(v);
                }
            }
            (Some(v), Entry::Occupied(mut e)) => {
                if v == *self.padding {
                    let _ = e.remove();
                } else {
                    e.insert(v);
                }
            }
        }
    }
}

/// None derive implementation.
///
/// # TODO for future
///
/// Currently [`Entry`] of [`pstd`] does not implement [`Debug`].<br/>
/// Therefore we are not using `derive` attribute at [`Debug`].
impl<T> Debug for ValueEditor<'_, T>
where
    T: Debug + PartialEq,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let type_name = util::name_of_type!(ValueEditor<'_, T>);
        let ek = self.entry.key();
        let ev = match self.entry.deref() {
            Entry::Vacant(_) => None,
            Entry::Occupied(x) => Some(x.get()),
        };

        f.debug_struct(type_name)
            .field(util::name_of!(padding in Self), &self.padding)
            .field(util::name_of!(entry in Self), &format!("{:?}", (ek, ev)))
            .field(util::name_of!(new_value in Self), &self.new_value)
            .finish()
    }
}
