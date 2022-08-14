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
            (5, 216_354_883_270_073_f64 / 98_802_063_360_000_f64),
            (6, 120f64 / 49f64),
            (7, 4_801_290_351_709_091_f64 / 1_778_437_140_480_000_f64),
            (8, 51_321_779_839_159_f64 / 17_435_658_240_000_f64),
            (9, 5_657_869_876_011_559_f64 / 1_778_437_140_480_000_f64),
            (100, 6_264_378_398_716_279_f64 / 1_778_437_140_480_000_f64),
        ];
        for (x, y) in test_cases {
            obj.apply(f64::from(x));
            let error = (obj.result() - y).abs();
            assert!(error < 1e-15);
        }
    }
}
