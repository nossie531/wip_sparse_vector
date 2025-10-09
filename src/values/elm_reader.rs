#[derive(Clone, Debug)]
pub struct ElmReader<'a, T>
where
    T: PartialEq,
{
    index: usize,
    value: &'a T,
}

impl<'a, T> ElmReader<'a, T>
where
    T: PartialEq,
{
    pub(crate) fn new(index: usize, value: &'a T) -> Self {
        Self { index, value }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn value(&self) -> &'a T {
        self.value
    }
}
