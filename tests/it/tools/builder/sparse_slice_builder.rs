use std::collections::BTreeSet;
use std::ops::Range;
use sparse_vector::prelude::*;
use crate::tools::builder::ValuesBuilder;

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
}

// Building methods.
impl SparseSliceBuilder {
    pub fn setup(&self) -> Context {
        let mut vec = SparseVec::with_padding(0, self.vb.padding());
        vec.extend(self.outside_values());
        Context::new(vec, self.range())
    }
}

// Reporting methods.
impl SparseSliceBuilder {
    pub fn range(&self) -> Range<usize> {
        let len = ValuesBuilder::new().len();
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

pub struct Context {
    vec: SparseVec<i32>,
    range: Range<usize>,
}

impl Context {
    pub fn build(&self) -> SparseSlice<'_, i32> {
        self.vec.slice(self.range.clone())
    }

    pub fn build_mut(&mut self) -> SparseSliceMut<'_, i32> {
        self.vec.slice_mut(self.range.clone())
    }

    fn new(vec: SparseVec<i32>, range: Range<usize>) -> Self {
        Self { vec, range }
    }
}
