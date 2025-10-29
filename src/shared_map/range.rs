use pstd::collections::btree_map::Range as StdRange;
use crate::shared_map::MapCell;

#[derive(Clone, Debug)]
pub(crate) struct Range<'a, K, V>(pub StdRange<'a, K, MapCell<V>>)
where
    K: 'a,
    V: 'a;

impl<'a, K, V> Iterator for Range<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| (x.0, x.1.get()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for Range<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|x| (x.0, x.1.get()))
    }
}