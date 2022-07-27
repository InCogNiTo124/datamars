use crate::ops::Mean;
use crate::ops::Operator;

pub struct GeoMean {
    mean: Mean,
}

impl Operator for GeoMean {
    fn apply(&mut self, x: f64) {
        // consider using log2, maybe it has better numerical properties?
        self.mean.apply(x.ln());
    }

    fn result(&self) -> f64 {
        self.mean.result().exp()
    }
}

impl GeoMean {
    pub const fn new() -> Self {
        Self { mean: Mean::new() }
    }
}
