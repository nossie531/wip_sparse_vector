use crate::shared_map::*;
use pstd::collections::btree_map::CursorMutKey as StdCursorMutKey;

pub struct CursorMutKey<'a, K, V>(pub StdCursorMutKey<'a, K, MapCell<V>>)
where
    K: 'a,
    V: 'a;

impl<'a, K, V> CursorMutKey<'a, K, V> {
    pub fn next(&mut self) -> Option<(&mut K, &mut V)> {
        self.0.next().map(|x| (x.0, x.1.get_mut()))
    }

    pub fn peek_prev(&mut self) -> Option<(&mut K, &mut V)> {
        self.0.peek_prev().map(|x| (x.0, x.1.get_mut()))
    }
}

impl<'a, K, V> CursorMutKey<'a, K, V>
where
    K: Ord,
{
    pub fn remove_prev(&mut self) -> Option<(K, V)> {
        self.0.remove_prev().map(|x| (x.0, x.1.into_inner()))
    }
}
