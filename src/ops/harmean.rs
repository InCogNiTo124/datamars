use crate::ops::{Mean, Operator};

pub struct HarMean {
    mean: Mean,
}

impl Operator for HarMean {
    fn apply(&mut self, x: f64) {
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

#[cfg(test)]
mod tests {
    use crate::{HarMean, Operator};

    #[test]
    fn test1() {
        let mut obj = HarMean::new();
        let test_cases = vec![
            (1, 1.0),
            (2, 4.0f64 / 3.0f64),
            (3, 18f64 / 11f64),
            (4, 1.920),
            (5, 216354883270073f64 / 98802063360000f64),
            (6, 120f64 / 49f64),
            (7, 4801290351709091f64 / 1778437140480000f64),
            (8, 51321779839159f64 / 17435658240000f64),
            (9, 5657869876011559f64/1778437140480000f64),
            (100, 6264378398716279f64/1778437140480000f64)
        ];
        for (x, y) in test_cases {
            obj.apply(x as f64);
            let error = (obj.result() - y).abs();
            assert!(error < 1e-15);
        }
    }
}

