use crate::for_test::SliceContext;
use crate::for_test::builders::ValuesBuilder;
use sparse_vector::prelude::*;
use std::collections::BTreeSet;
use std::ops::Range;

#[derive(Default)]
pub struct SparseSliceBuilder {
    vb: ValuesBuilder,
    heads: Vec<i32>,
    tails: Vec<i32>,
}

// Constructor and property methods.
impl SparseSliceBuilder {
    pub fn new() -> Self {
        Self {
            vb: Default::default(),
            heads: vec![42, 42, 42],
            tails: vec![42, 42, 42],
        }
    }

    pub fn padding(&self) -> i32 {
        self.vb.padding()
    }

    pub fn set_len(mut self, value: usize) -> Self {
        self.vb.set_len(value);
        self
    }
}

// Building methods.
impl SparseSliceBuilder {
    pub fn build(&self) -> SliceContext {
        let mut vec = SparseVec::with_padding(0, self.vb.padding());
        vec.extend(self.outside_values());
        SliceContext::new(vec, self.range())
    }
}

// Reporting methods.
impl SparseSliceBuilder {
    pub fn range(&self) -> Range<usize> {
        let len = self.vb.len();
        self.heads.len()..(self.heads.len() + len)
    }

    pub fn npad_indexs(&self) -> BTreeSet<usize> {
        ValuesBuilder::new().npad_indexs()
    }

    pub fn inside_values(&self) -> Vec<i32> {
        ValuesBuilder::new().values()
    }

    pub fn outside_values(&self) -> Vec<i32> {
        let heads = self.heads.clone();
        let tails = self.tails.clone();
        let bodys = self.vb.values();
        [heads, bodys, tails].concat()
    }
}
