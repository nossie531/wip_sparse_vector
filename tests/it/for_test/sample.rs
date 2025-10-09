use std::iter::{self, repeat_with};
use crate::for_test::template as tt;
use sparse_vec::prelude::*;

const RANDOM_TEST_SIZE: usize = 64;

pub fn default() -> SparseVec<i32> {
    SparseVec::default()
}

pub fn normal() -> SparseVec<i32> {
    tt::template().build()
}

pub fn normal_floats() -> SparseVec<f32> {
    tt::template().build_floats()
}

pub fn all_padding() -> SparseVec<i32> {
    tt::template().set_nnp(0).build()
}

pub fn random_trivals(seed: u64) -> SparseVec<i32> {
    tt::template().set_seed(seed).set_value_range(-1..=1).build()
}

pub fn pairs() -> impl Iterator<Item = [SparseVec<i32>;2]> {
    const CUSTOMS: [fn() -> [SparseVec<i32>;2]; 6] = [
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

    fn custom_pairs() -> impl Iterator<Item = [SparseVec<i32>;2]> {
        let mut customs = CUSTOMS.iter();
        iter::from_fn(move || {
            Some(customs.next()?())
        })
    }

    fn random_trivals_pairs() -> impl Iterator<Item = [SparseVec<i32>;2]> {
        let mut seed = 0;
        repeat_with(move || {
            let ret = random_trivals_pair(seed);
            seed += 1;
            ret
        }).take(RANDOM_TEST_SIZE)
    }

    fn normal_vs_normal() -> [SparseVec<i32>;2] {
        [normal(), normal()]
    }

    fn padding_vs_normal() -> [SparseVec<i32>;2] {
        let template = tt::template();
        let target_x = template.build();
        let mut target_y = template.build();
        let index = template.sample_padding_indexs(1)[0];
        *target_y.edit(index) += 1;
        [target_x, target_y]
    }

    fn value1_vs_value2() -> [SparseVec<i32>;2] {
        let template = tt::template();
        let target_x = template.build();
        let mut target_y = template.build();
        let index = template.sample_value_indexs(1)[0];
        *target_y.edit(index) += 1;
        [target_x, target_y]
    }

    fn padding1_vs_padding2() -> [SparseVec<i32>;2] {
        let mut target_x = SparseVec::<i32>::with_padding(0, 0);
        let mut target_y = SparseVec::<i32>::with_padding(0, 1);
        target_x.extend([0, 1, 0]);
        target_y.extend([0, 1, 0]);
        [target_x, target_y]
    }

    fn normal_vs_extra_padding() -> [SparseVec<i32>;2] {
        let target_x = normal();
        let mut target_y = normal();
        target_y.set_len(target_y.len() + 1);
        [target_x, target_y]
    }

    fn normal_vs_extra_value() -> [SparseVec<i32>;2] {
        let target_x = normal();
        let mut target_y = normal();
        target_y.extend([42]);
        [target_x, target_y]
    }

    fn random_trivals_pair(seed: u64) -> [SparseVec<i32>;2] {
        [random_trivals(seed), random_trivals(seed + 1)]
    }
}