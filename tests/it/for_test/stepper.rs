/// Step size iterator.
///
/// This iterator divides the range `0..m` into `n + 1` intervals
/// that are as equally separated as possible, and then retrieves
/// the `n` indices marking their boundaries.
pub struct Stepper {
    m: usize,
    n: usize,
    i: usize,
}

impl Stepper {
    pub fn new(m: usize, n: usize) -> Self {
        assert!(n <= m);
        Self { m, n, i: 0 }
    }

    pub fn diff(self) -> impl Iterator<Item = usize> {
        self.scan(None as Option<usize>, |s, x| {
            let ret = x - s.unwrap_or(0);
            *s = Some(x);
            Some(ret)
        })
    }
}

impl Iterator for Stepper {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.n {
            return None;
        }

        let width = self.m as f32 / (self.n + 1) as f32;
        let ret = (width * (self.i + 1) as f32).floor() as usize;
        self.i += 1;
        Some(ret)
    }
}
