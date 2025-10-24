use crate::for_test::SliceContext;
use crate::for_test::builders::*;
use crate::for_test::helper;
use crate::for_test::samples::*;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use sparse_vector::SparseVec;
use std::ops::RangeInclusive;

const SIDE_LEN: usize = 5;
const SIDE_VAL_RANGE: RangeInclusive<i32> = -2..=2;

pub struct SparseSliceSample();

impl SparseSliceSample {
    pub fn empty() -> SliceContext<i32> {
        let builder = SparseSliceBuilder::new().set_len(0);
        builder.build()
    }

    pub fn normal() -> SliceContext<i32> {
        let builder = SparseSliceBuilder::new();
        builder.build()
    }

    pub fn normal_floats() -> SliceContext<f32> {
        let builder = SparseSliceBuilder::new();
        builder.build_floats()
    }

    pub fn pairs() -> impl Iterator<Item = [SliceContext<i32>; 2]> {
        let vec_pairs = SparseVecSample::pairs();
        let mut rng = Pcg32::seed_from_u64(0);
        vec_pairs.map(move |[vec_x, vec_y]| {
            let sctx_x = Self::to_slice_context(vec_x, &mut rng);
            let sctx_y = Self::to_slice_context(vec_y, &mut rng);
            [sctx_x, sctx_y]
        })
    }

    fn to_slice_context<R: Rng>(mut src: SparseVec<i32>, rng: &mut R) -> SliceContext<i32> {
        let prologue = helper::rand_values(rng, SIDE_VAL_RANGE, SIDE_LEN);
        let epilogue = helper::rand_values(rng, SIDE_VAL_RANGE, SIDE_LEN);
        let range = prologue.len()..(prologue.len() + src.len());
        src.splice(0..0, prologue);
        src.extend(epilogue);
        SliceContext::new(src, range)
    }
}
