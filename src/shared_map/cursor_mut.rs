use pstd::collections::btree_map::CursorMut as StdCursorMut;
use crate::shared_map::*;

pub(crate) struct CursorMut<'a, K, V>(pub StdCursorMut<'a, K, MapCell<V>>)
where
    K: 'a,
    V: 'a;

impl<'a, K, V> CursorMut<'a, K, V> {
    pub fn next(&mut self) -> Option<(&K, &mut V)> {
        self.0.next().map(|x| (x.0, x.1.get_mut()))
    }

    pub fn remove_prev(&mut self) -> Option<(K, V)> {
        self.0.remove_prev().map(|x| (x.0, x.1.into_inner()))
    }

    pub unsafe fn with_mutable_key(self) -> CursorMutKey<'a, K, V> {
        CursorMutKey(self.0.with_mutable_key())
    }
}
