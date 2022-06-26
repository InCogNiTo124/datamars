pub use crate::operator::operator;
use crate::ops::mean::Mean;

pub struct HarMean {
    mean: Mean,
}

impl operator::Operator for HarMean {
    fn apply(&mut self, x: f64) {
        // consider using log2, maybe it has better numerical properties?
        self.mean.apply(x.recip());
    }

    fn result(&self) -> f64 {
        self.mean.result().recip()
    }
}

impl HarMean {
    pub fn new() -> HarMean {
        HarMean { mean: Mean::new() }
    }
}
