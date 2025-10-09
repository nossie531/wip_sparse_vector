pub struct Stepper {
    m: usize,
    n: usize,
    i: usize,
}

impl Stepper {
    pub fn new(m: usize, n: usize) -> Self {
        Self { m, n, i: 0 }
    }
}

impl Iterator for Stepper {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.n {
            return None;
        }

        let distance = self.m as f32 / (self.n + 1) as f32;
        let prev_pos = (distance * self.i as f32).floor() as isize;
        let curr_pos = (distance * (self.i + 1) as f32).floor() as isize;
        self.i += 1;
        Some((curr_pos - prev_pos - 1).max(0) as usize)
    }
}
