use crate::shared_map::MapCell;
use pstd::collections::btree_map::RangeMut as StdRangeMut;

#[derive(Debug)]
pub(crate) struct RangeMut<'a, K, V>(pub StdRangeMut<'a, K, MapCell<V>>)
where
    K: 'a,
    V: 'a;

impl<'a, K, V> Iterator for RangeMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| (x.0, x.1.get_mut()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for RangeMut<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|x| (x.0, x.1.get_mut()))
    }
}
