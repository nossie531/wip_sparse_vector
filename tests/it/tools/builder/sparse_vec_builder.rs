use crate::tools::builder::ValuesBuilder;
use sparse_vector::prelude::*;
use std::{collections::BTreeSet, ops::RangeInclusive};

#[derive(Default)]
pub struct SparseVecBuilder {
    vb: ValuesBuilder,
}

// Constructor and property methods.
impl SparseVecBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.vb.len()
    }

    pub fn nnp(&self) -> usize {
        self.vb.nnp()
    }

    pub fn padding(&self) -> i32 {
        self.vb.padding()
    }

    pub fn none_padding(&self) -> i32 {
        self.vb.none_padding()
    }

    pub fn set_seed(mut self, value: u64) -> Self {
        self.vb.set_seed(value);
        self
    }

    pub fn set_len(mut self, value: usize) -> Self {
        self.vb.set_len(value);
        self
    }

    pub fn set_nnp(mut self, value: usize) -> Self {
        self.vb.set_nnp(value);
        self
    }

    pub fn set_padding(mut self, value: i32) -> Self {
        self.vb.set_padding(value);
        self
    }

    pub fn set_value_range(mut self, value: RangeInclusive<i32>) -> Self {
        self.vb.set_value_range(value);
        self
    }
}

// Building methods.
impl SparseVecBuilder {
    pub fn build(&self) -> SparseVec<i32> {
        let mut ret = SparseVec::with_padding(0, self.vb.padding());
        ret.extend(self.values());
        ret
    }

    pub fn build_floats(&self) -> SparseVec<f32> {
        let mut ret = SparseVec::with_padding(0, self.vb.padding() as f32);
        ret.extend(self.values().iter().map(|x| *x as f32));
        ret
    }
}

impl SparseVecBuilder {
    pub fn values(&self) -> Vec<i32> {
        self.vb.values()
    }

    pub fn elms(&self) -> Vec<(usize, i32)> {
        self.vb.elms()
    }

    pub fn some_indexs(&self) -> Vec<usize> {
        self.vb.some_indexs()
    }

    pub fn npad_indexs(&self) -> BTreeSet<usize> {
        self.vb.npad_indexs()
    }

    pub fn some_pad_indexs(&self, n: usize) -> Vec<usize> {
        assert!(n < self.vb.len() - self.vb.nnp());
        self.vb.some_pad_indexs(n)
    }

    pub fn some_npad_indexs(&self, n: usize) -> Vec<usize> {
        assert!(n < self.vb.nnp());
        self.vb.some_npad_indexs(n)
    }
}