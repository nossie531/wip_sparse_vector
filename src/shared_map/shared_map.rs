use crate::shared_map::*;
use pstd::collections::BTreeMap;
use std::borrow::Borrow;
use std::ops::{Bound, RangeBounds};

#[derive(Clone, Debug)]
pub(crate) struct SharedMap<K, V>(BTreeMap<K, MapCell<V>>);

/// Methods like [`BTreeMap`].
impl<K, V> SharedMap<K, V> {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.0.get(key).map(|x| x.get())
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.0.get_mut(key).map(|x| x.get_mut())
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn last_entry(&mut self) -> Option<OccupiedEntry<'_, K, V>>
    where
        K: Ord,
    {
        self.0.last_entry().map(|x| OccupiedEntry(x))
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord,
    {
        let ret = self.0.insert(key, MapCell::new(value));
        ret.map(|x| x.into_inner())
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.0.remove(key).map(|x| x.into_inner())
    }

    pub fn range<T, R>(&self, range: R) -> Range<'_, K, V>
    where
        T: Ord + ?Sized,
        K: Borrow<T> + Ord,
        R: RangeBounds<T>,
    {
        Range(self.0.range(range))
    }

    pub fn range_mut<T, R>(&mut self, range: R) -> RangeMut<'_, K, V>
    where
        T: Ord + ?Sized,
        K: Borrow<T> + Ord,
        R: RangeBounds<T>,
    {
        RangeMut(self.0.range_mut(range))
    }
}

/// Methods like [`BTreeMap`] nightly.
impl<K, V> SharedMap<K, V> {
    pub fn lower_bound_mut<Q>(&mut self, bound: Bound<&Q>) -> CursorMut<'_, K, V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        CursorMut(self.0.lower_bound_mut(bound))
    }
}
