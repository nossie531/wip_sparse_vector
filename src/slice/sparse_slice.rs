use std::ops::Index;

pub trait SparseSlice<T>: Index<usize, Output = T> {
    /// Copies `self` into a new [`Vec`].
    fn to_vec(&self) -> Vec<T>
    where
        T: Clone;

    /// Fills `self` with elements by cloning `value`.
    fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        self.fill_with(|| value.clone());
    }

    /// Fills `self` with elements returned by calling a closure repeatedly.
    fn fill_with<F>(&mut self, f: F)
    where
        F: FnMut() -> T;

    /// Swaps two elements.
    ///
    /// # Panics
    ///
    /// Panics if `a` or `b` are out of bounds.
    fn swap(&mut self, a: usize, b: usize);
}
