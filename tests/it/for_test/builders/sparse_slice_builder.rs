//! Provider of [`SparseSliceBuilder`].

use crate::for_test::SliceContext;
use crate::for_test::builders::ValuesBuilder;
use sparse_vector::prelude::*;
use std::collections::BTreeSet;
use std::ops::Range;

/// Builder for [`SparseSlice`] and [`SparseSliceMut`].
#[derive(Default)]
pub struct SparseSliceBuilder {
    vb: ValuesBuilder,
    epilogue: Vec<i32>,
    prologue: Vec<i32>,
}

// Constructor and property methods.
impl SparseSliceBuilder {
    pub fn new() -> Self {
        Self {
            vb: Default::default(),
            epilogue: vec![42, 42, 42],
            prologue: vec![42, 42, 42],
        }
    }

    pub fn padding(&self) -> i32 {
        self.vb.padding()
    }

    pub fn none_padding(&self) -> i32 {
        self.vb.none_padding()
    }

    pub fn set_len(mut self, value: usize) -> Self {
        self.vb = self.vb.set_len(value);
        self
    }

    pub fn range(&self) -> Range<usize> {
        let len = self.vb.len();
        self.epilogue.len()..(self.epilogue.len() + len)
    }
}

// Building methods.
impl SparseSliceBuilder {
    pub fn build(&self) -> SliceContext<i32> {
        let mut vec = SparseVec::with_padding(0, self.vb.padding());
        vec.extend(self.vec_values());
        SliceContext::new(vec, self.range())
    }

    pub fn build_floats(&self) -> SliceContext<f32> {
        let mut vec = SparseVec::with_padding(0, self.vb.padding() as f32);
        vec.extend(self.vec_values().iter().map(|x| *x as f32));
        SliceContext::new(vec, self.range())
    }
}

// Reporting methods.
impl SparseSliceBuilder {
    pub fn slice_values(&self) -> Vec<i32> {
        self.vb.values()
    }

    pub fn vec_values(&self) -> Vec<i32> {
        let epilogue = self.epilogue.clone();
        let prologue = self.prologue.clone();
        let bodys = self.slice_values();
        [epilogue, bodys, prologue].concat()
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
