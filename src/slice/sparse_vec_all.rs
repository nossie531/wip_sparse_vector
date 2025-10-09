use std::ops::Index;
use crate::{SparseSlice, SparseVec};

#[repr(transparent)]
#[derive(Debug, Default)]
pub struct SparseVecAll<T>(SparseVec<T>)
where
    T: PartialEq;

impl<T> SparseVecAll<T>
where
    T: PartialEq,
{
    pub fn from_ref(r: &SparseVec<T>) -> &Self {
        unsafe { std::mem::transmute(r) }
    }

    pub fn from_mut(r: &mut SparseVec<T>) -> &mut Self {
        unsafe { std::mem::transmute(r) }
    }
}

impl<T> Index<usize> for SparseVecAll<T>
where 
    T: PartialEq,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.0.len());
        self.0.map.get(&index).unwrap_or(self.0.padding())
    }
}

impl<T> SparseSlice<T> for SparseVecAll<T>
where
    T: PartialEq,
{
    fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        Vec::from_iter(self.0.iter().cloned())
    }

    fn fill_with<F>(&mut self, mut f: F)
    where
        F: FnMut() -> T,
    {
        self.0.map.clear();

        for i in 0..self.0.len() {
            let value = f();
            if value != *self.0.padding() {
                self.0.map.insert(i, f());
            }
        }
    }

    fn swap(&mut self, a: usize, b: usize) {
        assert!(a < self.0.len());
        assert!(b < self.0.len());

        if a == b {
            return;
        }

        let a_val = self.0.map.remove(&a);
        let b_val = self.0.map.remove(&b);
        match (a_val, b_val) {
            (None, None) => {}
            (None, Some(b_val)) => {
                self.0.map.insert(a, b_val);
            }
            (Some(a_val), None) => {
                self.0.map.insert(b, a_val);
            }
            (Some(a_val), Some(b_val)) => {
                self.0.map.insert(a, b_val);
                self.0.map.insert(b, a_val);
            }
        }
    }
}

// TODO: スライス側にもトレイト実装が必要。
// というより、むしろこっちを本体にしないと…。
