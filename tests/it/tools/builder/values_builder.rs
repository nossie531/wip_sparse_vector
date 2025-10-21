use crate::tools::helper;
use crate::tools::stepper::Stepper;
use rand::SeedableRng;
use rand::seq::index::sample as rand_sample;
use rand_pcg::Pcg32;
use std::collections::BTreeSet;
use std::ops::RangeInclusive;

static LEN: usize = 16;
static NNP: usize = 5;
static PADDING: i32 = -1;
static VALUE_RANGE: RangeInclusive<i32> = -99..=99;

#[derive(Default)]
pub struct ValuesBuilder {
    seed: u64,
    len: Option<usize>,
    nnp: Option<usize>,
    padding: Option<i32>,
    value_range: Option<RangeInclusive<i32>>,
}

// Constructor and properties.
impl ValuesBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.len.unwrap_or(LEN)
    }

    pub fn nnp(&self) -> usize {
        let default_ratio = NNP as f32 / LEN as f32;
        let nnp_len = (self.len() as f32 * default_ratio) as usize;
        self.nnp.unwrap_or(nnp_len)
    }

    pub fn padding(&self) -> i32 {
        self.padding.as_ref().unwrap_or(&PADDING).clone()
    }

    pub fn none_padding(&self) -> i32 {
        self.padding().wrapping_sub(1)
    }

    pub fn value_range(&self) -> RangeInclusive<i32> {
        self.value_range.as_ref().unwrap_or(&VALUE_RANGE).clone()
    }

    pub fn set_seed(&mut self, value: u64) {
        self.seed = value;
    }

    pub fn set_len(&mut self, value: usize) {
        self.len = Some(value);
    }

    pub fn set_nnp(&mut self, value: usize) {
        self.nnp = Some(value);
    }

    pub fn set_padding(&mut self, value: i32) {
        self.padding = Some(value);
    }

    pub fn set_value_range(&mut self, value: RangeInclusive<i32>) {
        self.value_range = Some(value);
    }
}

// Reporting methods.
impl ValuesBuilder {
    pub fn values(&self) -> Vec<i32> {
        let mut ret = Vec::new();
        let rng = &mut Pcg32::seed_from_u64(self.seed);
        let indexs = self.npad_indexs();
        for index in 0..self.len() {
            let hit = indexs.contains(&index);
            let value = if hit {
                helper::rand_without(rng, self.value_range(), self.padding())
            } else {
                self.padding()
            };
            ret.push(value);
        }

        ret
    }

    pub fn array(&self) -> [i32; LEN] {
        assert_eq!(self.len(), LEN);
        <[_; LEN]>::try_from(self.values()).unwrap()
    }

    pub fn elms(&self) -> Vec<(usize, i32)> {
        let all = self.values().into_iter();
        let ret = all.enumerate().filter(|&(_, v)| v != self.padding());
        ret.collect()
    }

    pub fn pad_indexs(&self) -> BTreeSet<usize> {
        let all_indexs = BTreeSet::from_iter(0..self.len());
        &all_indexs - &self.npad_indexs()
    }

    pub fn npad_indexs(&self) -> BTreeSet<usize> {
        let rng = &mut Pcg32::seed_from_u64(self.seed);
        let ret = rand_sample(rng, self.len(), self.nnp());
        ret.iter().collect::<BTreeSet<_>>()
    }

    pub fn some_indexs(&self) -> Vec<usize> {
        let mut ret = Vec::new();
        let head_index = 0;
        let tail_index = self.len() - 1;
        let has_value = self.nnp() > 0;
        let has_padding = self.len() - self.nnp() > 0;
        let some_value_indexs = has_value.then(|| self.some_npad_indexs(1)[0]);
        let some_padding_indexs = has_padding.then(|| self.some_pad_indexs(1)[0]);
        ret.extend([head_index, tail_index]);
        ret.extend(some_value_indexs);
        ret.extend(some_padding_indexs);
        ret
    }

    pub fn some_pad_indexs(&self, n: usize) -> Vec<usize> {
        assert!(n < self.len() - self.nnp());
        let mut ret = Vec::new();
        let indexs = self.pad_indexs();
        let indexs = &mut indexs.iter();
        let stepper = Stepper::new(self.len() - self.nnp(), n);
        for step in stepper.diff() {
            ret.push(*indexs.nth(step).unwrap());
        }

        ret
    }

    pub fn some_npad_indexs(&self, n: usize) -> Vec<usize> {
        assert!(n < self.nnp());
        let mut ret = Vec::new();
        let indexs = self.npad_indexs();
        let indexs = &mut indexs.iter();
        let stepper = Stepper::new(self.nnp(), n);
        for step in stepper.diff() {
            ret.push(*indexs.nth(step).unwrap());
        }

        ret
    }
}
