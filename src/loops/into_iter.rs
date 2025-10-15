use crate::util;
use only_one::prelude::*;
use pstd::collections::btree_map::{BTreeMap, IntoIter as MapIter};
use std::fmt::Debug;
use std::iter::FusedIterator;
use std::ops::Range;

pub struct IntoIter<T>
where
    T: PartialEq,
{
    padding: One<T>,
    filler: One<fn(&T) -> T>,
    iter: MapIter<usize, T>,
    range: Range<usize>,
    head_memo: Option<(usize, T)>,
    tail_memo: Option<(usize, T)>,
}

impl<T> IntoIter<T>
where
    T: PartialEq,
{
    pub(crate) fn new(
        len: usize,
        padding: T,
        filler: fn(&T) -> T,
        iter: MapIter<usize, T>,
    ) -> Self {
        Self {
            padding: One::new(padding),
            filler: One::new(filler),
            iter,
            range: 0..len,
            head_memo: None,
            tail_memo: None,
        }
    }

    fn is_end(&self) -> bool {
        self.size_hint().1.unwrap() == 0
    }
}

impl<T> Default for IntoIter<T>
where
    T: PartialEq,
{
    fn default() -> Self {
        Self {
            padding: Default::default(),
            filler: Default::default(),
            iter: BTreeMap::new().into_iter(),
            range: Default::default(),
            head_memo: Default::default(),
            tail_memo: Default::default(),
        }
    }
}

impl<T> ExactSizeIterator for IntoIter<T>
where
    T: PartialEq,
{
    // nop.
}

impl<T> FusedIterator for IntoIter<T>
where
    T: PartialEq,
{
    // nop.
}

impl<T> Iterator for IntoIter<T>
where
    T: PartialEq,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end() {
            return None;
        }

        let head_memo = self.head_memo.as_ref();
        if head_memo.is_none() {
            self.head_memo = self.iter.next();
        }

        let head_memo = self.head_memo.as_ref();
        let tail_memo = self.tail_memo.as_ref();
        let hit_head = head_memo.is_some_and(|x| x.0 == self.range.start);
        let hit_tail = tail_memo.is_some_and(|x| x.0 == self.range.start);
        let ret = match (hit_head, hit_tail) {
            (true, _) => self.head_memo.take().unwrap().1,
            (_, true) => self.tail_memo.take().unwrap().1,
            _ => (self.filler)(&self.padding),
        };

        self.range.start += 1;
        Some(ret)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.range.len(), Some(self.range.len()))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T>
where
    T: PartialEq,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.is_end() {
            return None;
        }

        let tail_memo = self.tail_memo.as_ref();
        if tail_memo.is_none() {
            self.tail_memo = self.iter.next_back();
        }

        let tail_pos = self.range.end.checked_sub(1);
        let tail_memo = self.tail_memo.as_ref();
        let head_memo = self.head_memo.as_ref();
        let hit_tail = tail_memo.is_some_and(|x| Some(x.0) == tail_pos);
        let hit_head = head_memo.is_some_and(|x| Some(x.0) == tail_pos);
        let ret = match (hit_tail, hit_head) {
            (true, _) => self.tail_memo.take().unwrap().1,
            (_, true) => self.head_memo.take().unwrap().1,
            _ => (self.filler)(&self.padding),
        };

        self.range.end -= 1;
        Some(ret)
    }
}

/// None derive implementation.
///
/// # TODO for future
///
/// Currently [`IntoIter`](MapIter) of [`pstd`] does not implement [`Debug`].
/// <br/> Therefore we are not using `derive` attribute at [`Debug`].
impl<T> Debug for IntoIter<T>
where
    T: Debug + PartialEq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = util::name_of_type!(IntoIter<T>);
        f.debug_struct(type_name)
            .field(util::name_of!(padding in Self), &self.padding)
            .field(util::name_of!(range in Self), &self.range)
            .field(util::name_of!(head_memo in Self), &self.head_memo)
            .field(util::name_of!(tail_memo in Self), &self.tail_memo)
            .finish_non_exhaustive()
    }
}
