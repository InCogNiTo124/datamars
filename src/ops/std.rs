use crate::{Mean, Operator};

pub struct Std {
    mean: Mean,
    result: f64,
}

impl Operator for Std {
    fn apply(&mut self, x: f64) {
        let old_mean = self.mean.result();
        self.mean.apply(x);
        let new_mean = self.mean.result();
        self.result += (x - old_mean) * (x - new_mean);
    }

    fn result(&self) -> f64 {
        (self.result / (self.mean.n as f64)).sqrt()
    }
}

impl Std {
    pub const fn new() -> Self {
        Self {
            mean: Mean::new(),
            result: 0.0,
        }
    }
}
