//! Samples for range.

use std::ops::{Bound, Range, RangeBounds};

/// Create range maker for specified collection length.
pub fn range_for(len: usize) -> RangeFor {
    RangeFor { len }
}

/// [`Range`] sample creator.
pub struct RangeFor {
    len: usize
}

impl RangeFor {
    /// Returns sample empty range.
    pub fn empty(&self) -> Range<usize> {
        let start = self.len / 2;
        let end = self.len / 2;
        start..end
    }

    /// Returns sample reverse order range.
    pub fn rev_order(&self) -> impl RangeBounds<usize> + 'static {
        let start = Bound::Excluded(self.len / 2);
        let end = Bound::Excluded(self.len / 2);
        (start, end)
    }

    /// Returns sample out bounds range.
    pub fn out_bounds(&self) -> Range<usize> {
        let start = self.len / 2;
        let end = self.len + 1;
        start..end
    }

    /// Returns sample normal range.
    pub fn normal(&self) -> Range<usize> {
        let start = (self.len as f32 * 1.0 / 3.0).round();
        let range_len = (self.len as f32 / 3.0).round();
        (start as usize)..((start + range_len) as usize)
    }

    /// Returns sample range including specified index.
    pub fn include(&self, index: usize) -> Range<usize> {
        let self_len = self.len as f32;
        let range_len = (self_len / 3.0).round();
        let bgn = (index as f32 - range_len / 2.0).round();
        let end = (index as f32 - range_len / 2.0).round() + range_len;
        let bgn_ob = f32::max(0.0, 0.0 - bgn);
        let end_ob = f32::max(0.0, end - self_len);
        let start = bgn + bgn_ob - end_ob;
        (start as usize)..((start + range_len) as usize)
    }

    /// Returns sample range with specified length.
    pub fn with_len(&self, len: usize) -> Range<usize> {
        assert!(len <= self.len);    
        let side_len = (self.len - len) / 2;
        side_len..(side_len + len)
    }
}
