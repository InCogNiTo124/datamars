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

#[cfg(test)]
mod tests {
    use crate::{Operator, Std};

    #[test]
    fn test1() {
        let mut obj = Std::new();
        let test_cases = vec![
            (1, 0.0),
            (2, 0.5),
            (3, (2f64 / 3f64).sqrt()),
            (4, 1.25f64.sqrt()),
            (5, 2f64.sqrt()),
            (6, (35f64 / 12f64).sqrt()),
            (7, 2.0),
            (8, 5.25f64.sqrt()),
            (9, (20f64 / 3f64).sqrt()),
            (100, 818.25f64.sqrt()),
        ];
        for (x, y) in test_cases {
            obj.apply(x as f64);
            let error = (obj.result() - y).abs();
            assert!(error < 1e-15);
        }
    }
}
