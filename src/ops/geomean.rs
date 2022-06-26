pub use crate::operator::operator;
use crate::ops::mean::Mean;

pub struct GeoMean {
    mean: Mean,
}

impl operator::Operator for GeoMean {
    fn apply(&mut self, x: f64) {
        // consider using log2, maybe it has better numerical properties?
        self.mean.apply(x.ln());
    }

    fn result(&self) -> f64 {
        self.mean.result().exp()
    }
}

impl GeoMean {
    pub fn new() -> GeoMean {
        GeoMean { mean: Mean::new() }
    }
}
