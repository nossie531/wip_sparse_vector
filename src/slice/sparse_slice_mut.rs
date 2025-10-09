use crate::slice::SparseSlice;
use crate::iter::SparseWriter;
use crate::values::ValueEditor;

pub trait SparseSliceMut<T>: SparseSlice<T>
where 
    T: PartialEq,
{
    /// Returns none padding elements writer.
    fn sparse_writer(&mut self) -> SparseWriter<'_, T>;

    /// Takes the value of index, leaving padding value.
    /// 
    /// # Panics
    /// 
    /// Panics if `index` is not less than vector length.
    fn take(&mut self, index: usize) -> Option<T>;

    /// Returns value editor.
    ///
    /// # Panics
    ///
    /// Panics if `index` is not less than vector length.
    fn edit(&mut self, index: usize) -> ValueEditor<'_, T>;

    /// Fills `self` with elements by cloning `value`.
    fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        self.fill_with(|| value.clone());
    }

    /// Fills `self` with elements returned by calling a closure repeatedly.
    fn fill_with<F>(&mut self, mut f: F)
    where
        F: FnMut() -> T,
    {
        for i in 0..self.len() {
            let value = f();
            *self.edit(i) = value;
        }
    }

    /// Swaps two elements.
    ///
    /// # Panics
    ///
    /// Panics if `a` or `b` are out of bounds.
    fn swap(&mut self, x: usize, y: usize) {
        assert!(x < self.len());
        assert!(y < self.len());

        if x == y {
            return;
        }

        let xv = self.take(x);
        let yv = self.take(y);
        match (xv, yv) {
            (None, None) => {},
            (None, Some(bv)) => {
                *self.edit(x) = bv;
            },
            (Some(av), None) => {
                *self.edit(y) = av;
            },
            (Some(av), Some(bv)) => {
                *self.edit(x) = bv;
                *self.edit(y) = av;
            },
        }
    }
}
