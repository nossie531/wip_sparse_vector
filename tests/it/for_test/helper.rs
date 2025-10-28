//! Helper methods for test.

use rand::Rng;
use std::collections::HashSet;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::RangeInclusive;

/// Returns some value not contained in `values`.
pub fn some_other_of<I>(values: I) -> i32
where
    I: IntoIterator<Item = i32>,
{
    let set = values.into_iter().collect::<HashSet<_>>();
    for val in 0.. {
        if set.contains(&val) {
            continue;
        }
        return val;
    }

    unreachable!()
}

/// Consume all items of iterator.
pub fn consume_all_items<I>(values: &mut I, flag: bool)
where
    I: Iterator,
{
    if flag {
        let _ = values.all(|_| true);
    }
}

/// Returns target hash value with default hasher.
pub fn hash<T: Hash>(target: &T) -> u64 {
    let hasher = &mut DefaultHasher::new();
    target.hash(hasher);
    hasher.finish()
}

/// Returns a random value within the range excluding `na`.
pub fn rand_without<R: Rng>(rng: &mut R, range: RangeInclusive<i32>, na: i32) -> i32 {
    let adjusted_range = *range.start()..=(*range.end() - 1);
    let trial = rng.random_range(adjusted_range);
    if trial != na { trial } else { *range.end() }
}

/// Returns random values within the range.
pub fn rand_values<R: Rng>(rng: &mut R, range: RangeInclusive<i32>, n: usize) -> Vec<i32> {
    let mut ret = Vec::new();
    let len = rng.random_range(0..=n);
    for _ in 0..len {
        ret.push(rng.random_range(range.clone()));
    }

    ret
}
