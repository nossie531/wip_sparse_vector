use crate::for_test::stepper::Stepper;
use rand::seq::index::sample as rand_sample;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use sparse_vector::prelude::*;
use std::collections::BTreeSet;
use std::ops::RangeInclusive;

pub static LEN: usize = 10;
pub static NNP: usize = 5;
pub static PADDING: i32 = -1;
pub static VALUE_RANGE: RangeInclusive<i32> = -10..=10;

pub fn template() -> Template {
    Template::new()
}

#[derive(Default)]
pub struct Template {
    seed: u64,
    len: Option<usize>,
    nnp: Option<usize>,
    padding: Option<i32>,
    value_range: Option<RangeInclusive<i32>>,
}

impl Template {
    pub fn len(&self) -> usize {
        self.len.unwrap_or(LEN)
    }

    pub fn nnp(&self) -> usize {
        let default_ratio = NNP as f32 / LEN as f32;
        self.nnp
            .unwrap_or((self.len() as f32 * default_ratio) as usize)
    }

    pub fn padding(&self) -> i32 {
        self.padding.as_ref().unwrap_or(&PADDING).clone()
    }

    pub fn value_range(&self) -> RangeInclusive<i32> {
        self.value_range.as_ref().unwrap_or(&VALUE_RANGE).clone()
    }

    pub fn sample_indexs(&self) -> Vec<usize> {
        let mut ret = Vec::new();
        let head_index = 0;
        let tail_index = self.len() - 1;
        let has_value = self.nnp() > 0;
        let has_padding = self.len() - self.nnp() > 0;
        let some_value_indexs = has_value.then(|| self.sample_value_indexs(1)[0]);
        let some_padding_indexs = has_padding.then(|| self.sample_padding_indexs(1)[0]);
        ret.extend([head_index, tail_index]);
        ret.extend(some_value_indexs);
        ret.extend(some_padding_indexs);
        ret
    }

    pub fn sample_value_indexs(&self, n: usize) -> Vec<usize> {
        assert!(n < self.nnp());
        let mut ret = Vec::new();
        let indexs = self.sample_values_set();
        let indexs = &mut indexs.iter();
        let stepper = Stepper::new(self.nnp(), n);
        for step in stepper.diff() {
            ret.push(*indexs.nth(step).unwrap());
        }

        ret
    }

    pub fn sample_padding_indexs(&self, n: usize) -> Vec<usize> {
        assert!(n < self.len() - self.nnp());
        let mut ret = Vec::new();
        let indexs = self.sample_padding_set();
        let indexs = &mut indexs.iter();
        let stepper = Stepper::new(self.len() - self.nnp(), n);
        for step in stepper.diff() {
            ret.push(*indexs.nth(step).unwrap());
        }

        ret
    }

    pub fn sample_values_set(&self) -> BTreeSet<usize> {
        let rng = &mut Pcg32::seed_from_u64(1);
        rand_sample(rng, self.len(), self.nnp())
            .iter()
            .collect::<BTreeSet<_>>()
    }

    pub fn sample_padding_set(&self) -> BTreeSet<usize> {
        let all_indexs = BTreeSet::from_iter(0..self.len());
        &all_indexs - &self.sample_values_set()
    }

    pub fn sample_elms(&self) -> Vec<(usize, i32)> {
        self.sample_vec()
            .into_iter()
            .enumerate()
            .filter(|&(_, v)| v != self.padding())
            .collect()
    }

    pub fn sample_arr(&self) -> [i32; LEN] {
        assert_eq!(self.len(), LEN);
        <[_; LEN]>::try_from(self.sample_vec()).unwrap()
    }

    pub fn sample_vec(&self) -> Vec<i32> {
        let mut ret = Vec::new();
        let rng = &mut Pcg32::seed_from_u64(self.seed);
        let indexs = self.sample_values_set();
        for index in 0..self.len() {
            let hit = indexs.contains(&index);
            let value = if hit {
                self.rand_without(rng, self.padding())
            } else {
                self.padding()
            };
            ret.push(value);
        }

        ret
    }

    pub fn build(&self) -> SparseVec<i32> {
        let mut ret = SparseVec::with_padding(self.len(), self.padding());
        let src = self.sample_vec();

        for index in 0..self.len() {
            *ret.edit(index) = src[index];
        }

        ret
    }

    pub fn build_floats(&self) -> SparseVec<f32> {
        let mut ret = SparseVec::with_padding(self.len(), self.padding() as f32);
        let src = self.sample_vec();

        for index in 0..self.len() {
            *ret.edit(index) = src[index] as f32;
        }

        ret
    }

    pub fn set_seed(mut self, value: u64) -> Self {
        self.seed = value;
        self
    }

    pub fn set_len(mut self, value: usize) -> Self {
        self.len = Some(value);
        self
    }

    pub fn set_nnp(mut self, value: usize) -> Self {
        self.nnp = Some(value);
        self
    }

    pub fn set_padding(mut self, value: i32) -> Self {
        self.padding = Some(value);
        self
    }

    pub fn set_value_range(mut self, value: RangeInclusive<i32>) -> Self {
        self.value_range = Some(value);
        self
    }

    fn new() -> Self {
        Default::default()
    }

    fn rand_without<T: Rng>(&self, rng: &mut T, na: i32) -> i32 {
        let original_range = self.value_range();
        let adjusted_range = *original_range.start()..=(*original_range.end() - 1);
        let trial = rng.random_range(adjusted_range);
        if trial != na {
            trial
        } else {
            *original_range.end()
        }
    }
}
