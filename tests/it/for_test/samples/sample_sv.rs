use crate::for_test::builders::*;
use sparse_vector::prelude::*;
use std::iter::{self, repeat_with};

const RANDOM_TEST_SIZE: usize = 64;

pub fn default() -> SparseVec<i32> {
    SparseVec::default()
}

pub fn normal() -> SparseVec<i32> {
    SparseVecBuilder::new().build()
}

pub fn single() -> SparseVec<i32> {
    SparseVecBuilder::new().set_len(1).set_nnp(1).build()
}

pub fn normal_floats() -> SparseVec<f32> {
    SparseVecBuilder::new().build_floats()
}

pub fn all_padding() -> SparseVec<i32> {
    SparseVecBuilder::new().set_nnp(0).build()
}

pub fn random_trivals(seed: u64) -> SparseVec<i32> {
    SparseVecBuilder::new()
        .set_seed(seed)
        .set_value_range(-1..=1)
        .build()
}

pub fn pairs() -> impl Iterator<Item = [SparseVec<i32>; 2]> {
    const CUSTOMS: [fn() -> [SparseVec<i32>; 2]; 8] = [
        default_vs_default,
        single_vs_single,
        normal_vs_normal,
        padding_vs_normal,
        value1_vs_value2,
        padding1_vs_padding2,
        normal_vs_extra_padding,
        normal_vs_extra_value,
    ];

    let customs = custom_pairs();
    let randoms = random_trivals_pairs();
    return customs.chain(randoms);

    fn custom_pairs() -> impl Iterator<Item = [SparseVec<i32>; 2]> {
        let mut customs = CUSTOMS.iter();
        iter::from_fn(move || Some(customs.next()?()))
    }

    fn random_trivals_pairs() -> impl Iterator<Item = [SparseVec<i32>; 2]> {
        let mut seed = 0;
        repeat_with(move || {
            let ret = random_trivals_pair(seed);
            seed += 1;
            ret
        })
        .take(RANDOM_TEST_SIZE)
    }

    fn default_vs_default() -> [SparseVec<i32>; 2] {
        [default(), default()]
    }

    fn single_vs_single() -> [SparseVec<i32>; 2] {
        [single(), single()]
    }

    fn normal_vs_normal() -> [SparseVec<i32>; 2] {
        [normal(), normal()]
    }

    fn padding_vs_normal() -> [SparseVec<i32>; 2] {
        let builder = SparseVecBuilder::new();
        let target_x = builder.build();
        let mut target_y = builder.build();
        let index = builder.some_pad_indexs(1)[0];
        *target_y.edit(index) += 1;
        [target_x, target_y]
    }

    fn value1_vs_value2() -> [SparseVec<i32>; 2] {
        let builder = SparseVecBuilder::new();
        let target_x = builder.build();
        let mut target_y = builder.build();
        let index = builder.some_npad_indexs(1)[0];
        *target_y.edit(index) += 1;
        [target_x, target_y]
    }

    fn padding1_vs_padding2() -> [SparseVec<i32>; 2] {
        let mut target_x = SparseVec::<i32>::with_padding(0, 0);
        let mut target_y = SparseVec::<i32>::with_padding(0, 1);
        target_x.extend([0, 1, 0]);
        target_y.extend([0, 1, 0]);
        [target_x, target_y]
    }

    fn normal_vs_extra_padding() -> [SparseVec<i32>; 2] {
        let target_x = normal();
        let mut target_y = normal();
        target_y.set_len(target_y.len() + 1);
        [target_x, target_y]
    }

    fn normal_vs_extra_value() -> [SparseVec<i32>; 2] {
        let target_x = normal();
        let mut target_y = normal();
        target_y.extend([42]);
        [target_x, target_y]
    }

    fn random_trivals_pair(seed: u64) -> [SparseVec<i32>; 2] {
        [random_trivals(seed), random_trivals(seed + 1)]
    }
}
