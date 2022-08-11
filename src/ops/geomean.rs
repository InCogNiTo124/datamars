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

#[cfg(test)]
mod tests {
    use crate::{GeoMean, Operator};

    #[test]
    fn test1() {
        let mut obj = GeoMean::new();
        let test_cases = vec![
            (1, 1.0),
            (2, (2.0 as f64).sqrt()),
            (3, (6.0 as f64).cbrt()),
            (4, (24 as f64).powf(1.0f64/4.0f64)),
            (5, (120 as f64).powf(1.0f64/5.0f64)),
            (6, (720 as f64).powf(1.0f64/6.0f64)),
            (7, (5040 as f64).powf(1.0f64/7.0f64)),
            (8, (40320 as f64).powf(1.0f64/8.0f64)),
            (9, (362880 as f64).powf(1.0f64/9.0f64)),
            (100, (36288000 as f64).powf(1.0f64/10.0f64)),
        ];
        for (x, y) in test_cases {
            obj.apply(x as f64);
            let error = (obj.result() - y).abs();
            assert!(error < 1e-15);
        }
    }
}
