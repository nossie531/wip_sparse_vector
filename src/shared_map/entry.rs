use crate::shared_map::MapCell;
use pstd::collections::btree_map::OccupiedEntry as StdOccupiedEntry;

pub(crate) struct OccupiedEntry<'a, K, V>(pub StdOccupiedEntry<'a, K, MapCell<V>>);

impl<'a, K, V> OccupiedEntry<'a, K, V>
where
    K: Ord,
{
    pub fn key(&self) -> &K {
        self.0.key()
    }

    pub fn remove(self) -> V {
        self.0.remove().into_inner()
    }
}
