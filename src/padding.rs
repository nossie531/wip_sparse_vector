//! Provider of [`Padding`].

/// Padding value source.
#[derive(Clone, Debug)]
pub struct Padding<T> {
    /// Padding value.
    value: T,

    /// Clone method for padding value.
    clone_value: fn(&T) -> T,
}

impl<T> Padding<T> {
    /// Create a new instance by default mode.
    pub fn by_default() -> Self
    where
        T: Default,
    {
        Self {
            value: Default::default(),
            clone_value: Self::clone_default,
        }
    }

    /// Create a new instance by clone mode.
    pub fn by_clone(value: T) -> Self
    where
        T: Clone,
    {
        Self {
            value,
            clone_value: T::clone,
        }
    }

    /// Returns padding value reference.
    pub fn refs(&self) -> &T {
        &self.value
    }

    /// Returns padding value.
    pub fn value(&self) -> T {
        (self.clone_value)(&self.value)
    }

    /// Clone method for default value.
    fn clone_default(_x: &T) -> T
    where
        T: Default,
    {
        T::default()
    }
}
