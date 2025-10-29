use std::{borrow::Borrow, ops::{Bound, RangeBounds}};
use pstd::collections::BTreeMap;
use crate::shared_map::*;

#[derive(Clone, Debug)]
pub(crate) struct SharedMap<K, V> {
    base: BTreeMap<K, MapCell<V>>,
}

/// Methods like [`BTreeMap`].
impl<K, V> SharedMap<K, V> {
    pub fn new() -> Self {
        Self { base: Default::default() }
    }

    pub fn len(&self) -> usize {
        self.base.len()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.base.get(key).map(|x| x.get())
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.base.get_mut(key).map(|x| x.get_mut())
    }

    pub fn clear(&mut self) {
        self.base.clear();
    }

    pub fn last_entry(&mut self) -> Option<OccupiedEntry<'_, K, V>>
    where
        K: Ord,
    {
        self.base.last_entry().map(|x| OccupiedEntry(x))
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord,
    {
        let ret = self.base.insert(key, MapCell::new(value));
        ret.map(|x| x.into_inner())
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.base.remove(key).map(|x| x.into_inner())
    }

    pub fn range<T, R>(&self, range: R) -> Range<'_, K, V>
    where
        T: Ord + ?Sized,
        K: Borrow<T> + Ord,
        R: RangeBounds<T>,
    {
        Range(self.base.range(range))
    }

    pub fn range_mut<T, R>(&mut self, range: R) -> RangeMut<'_, K, V>
    where
        T: Ord + ?Sized,
        K: Borrow<T> + Ord,
        R: RangeBounds<T>,
    {
        RangeMut(self.base.range_mut(range))
    }
}

/// Methods like [`BTreeMap`] nightly.
impl<K, V> SharedMap<K, V> {
    pub fn lower_bound_mut<Q>(&mut self, bound: Bound<&Q>) -> CursorMut<'_, K, V>
        where
            K: Borrow<Q> + Ord,
            Q: Ord + ?Sized,
    {
        CursorMut(self.base.lower_bound_mut(bound))
    }
}
