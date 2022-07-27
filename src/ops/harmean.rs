use crate::ops::{Mean, Operator};

pub struct HarMean {
    mean: Mean,
}

impl Operator for HarMean {
    fn apply(&mut self, x: f64) {
        // consider using log2, maybe it has better numerical properties?
        self.mean.apply(x.recip());
    }

    fn result(&self) -> f64 {
        self.mean.result().recip()
    }
}

impl HarMean {
    pub const fn new() -> Self {
        Self { mean: Mean::new() }
    }
}
