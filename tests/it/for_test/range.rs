use std::ops::{Bound, Range, RangeBounds};

pub fn empty(len: usize) -> Range<usize> {
    let start = len / 2;
    let end = len / 2;
    start..end
}

pub fn rev_order(len: usize) -> impl RangeBounds<usize> {
    let start = Bound::Excluded(len / 2);
    let end = Bound::Excluded(len / 2);
    (start, end)
}

pub fn gt_len(len: usize) -> Range<usize> {
    let start = len / 2;
    let end = len + 1;
    start..end
}

pub fn normal(len: usize) -> Range<usize> {
    let div = len as f32 / 3.0;
    let min = div.round() as usize;
    let max = (div * 2.0).round() as usize;
    min..((max + 1).min(len))
}

pub fn len_in(range_len: usize, len: usize) -> Range<usize> {
    assert!(range_len <= len);    
    let side_len = (len - range_len) / 2;
    side_len..(side_len + range_len)
}
