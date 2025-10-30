use std::cell::UnsafeCell;

#[derive(Debug)]
pub(crate) struct MapCell<T>(UnsafeCell<T>);

impl<T> MapCell<T> {
    pub fn new(value: T) -> Self {
        Self(UnsafeCell::new(value))
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.0.get() }
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut()
    }

    pub fn into_inner(self) -> T {
        self.0.into_inner()
    }
}

impl<T> Clone for MapCell<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(UnsafeCell::new(self.get().clone()))
    }
}
