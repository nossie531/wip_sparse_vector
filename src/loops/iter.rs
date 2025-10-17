use crate::alias::MapRange;
use crate::msg;
use crate::prelude::*;
use only_one::prelude::*;
use std::iter::FusedIterator;
use std::ops::Range;

#[derive(Debug)]
#[must_use = msg::iter_must_use!()]
pub struct Iter<'a, T>
where
    T: PartialEq,
{
    padding: One<&'a T>,
    map_range: One<MapRange<'a, T>>,
    idx_range: Range<usize>,
    head_memo: Option<(&'a usize, &'a T)>,
    tail_memo: Option<(&'a usize, &'a T)>,
}

impl<'a, T> Iter<'a, T>
where
    T: PartialEq,
{
    pub(crate) fn new(vec: &'a SparseVec<T>, range: Range<usize>) -> Self {
        Self {
            padding: One::new(&vec.padding),
            map_range: One::new(vec.map.range(range.clone())),
            idx_range: range,
            head_memo: None,
            tail_memo: None,
        }
    }

    fn is_end(&self) -> bool {
        self.size_hint().1.unwrap() == 0
    }

    fn iter(&mut self) -> &mut MapRange<'a, T> {
        &mut self.map_range
    }
}

/// Restricted implementation.
///
/// # TODO for future
///
/// Currently `T` requires [`Clone`]. This is current limitation.
impl<T> Clone for Iter<'_, T>
where
    T: PartialEq + Clone,
{
    fn clone(&self) -> Self {
        Self {
            padding: self.padding,
            map_range: self.map_range.clone(),
            idx_range: self.idx_range.clone(),
            head_memo: self.head_memo,
            tail_memo: self.tail_memo,
        }
    }
}

impl<T> Default for Iter<'_, T>
where
    T: PartialEq,
{
    fn default() -> Self {
        Self {
            padding: Default::default(),
            map_range: Default::default(),
            idx_range: Default::default(),
            head_memo: Default::default(),
            tail_memo: Default::default(),
        }
    }
}

impl<T> ExactSizeIterator for Iter<'_, T>
where
    T: PartialEq,
{
    // nop.
}

impl<T> FusedIterator for Iter<'_, T>
where
    T: PartialEq,
{
    // nop.
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: PartialEq,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end() {
            return None;
        }

        let head_memo = self.head_memo.as_ref();
        if head_memo.is_none() {
            self.head_memo = self.iter().next();
        }

        let head_memo = self.head_memo.as_ref();
        let tail_memo = self.tail_memo.as_ref();
        let hit_head = head_memo.is_some_and(|x| *x.0 == self.idx_range.start);
        let hit_tail = tail_memo.is_some_and(|x| *x.0 == self.idx_range.start);
        let ret = match (hit_head, hit_tail) {
            (true, _) => self.head_memo.take().unwrap().1,
            (_, true) => self.tail_memo.take().unwrap().1,
            _ => *self.padding,
        };

        self.idx_range.start += 1;
        Some(ret)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.idx_range.len(), Some(self.idx_range.len()))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T>
where
    T: PartialEq,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.is_end() {
            return None;
        }

        let tail_memo = self.tail_memo.as_ref();
        if tail_memo.is_none() {
            self.tail_memo = self.iter().next_back();
        }

        let tail_pos = self.idx_range.end.checked_sub(1);
        let tail_memo = self.tail_memo.as_ref();
        let head_memo = self.head_memo.as_ref();
        let hit_tail = tail_memo.is_some_and(|x| Some(*x.0) == tail_pos);
        let hit_head = head_memo.is_some_and(|x| Some(*x.0) == tail_pos);
        let ret = match (hit_tail, hit_head) {
            (true, _) => self.tail_memo.take().unwrap().1,
            (_, true) => self.head_memo.take().unwrap().1,
            _ => *self.padding,
        };

        self.idx_range.end -= 1;
        Some(ret)
    }
}
