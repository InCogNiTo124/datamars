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
    use crate::{Std, Operator};

    #[test]
    fn test1() {
        let mut obj = Std::new();
        let test_cases = vec![
            (1, 0.0),
            (2, 0.5),
            (3, (2.0/3.0 as f64).sqrt()),
            (4, (1.25 as f64).sqrt()),
            (5, (2.0 as f64).sqrt()),
            (6, (35.0/12.0 as f64).sqrt()),
            (7, 2.0),
            (8, (5.25 as f64).sqrt()),
            (9, (20.0/3 as f64).sqrt()),
            (100, (818.25 as f64).sqrt()),
        ];
        for (x, y) in test_cases {
            obj.apply(x as f64);
            let error = (obj.result() - y).abs();
            assert!(error < 1e-15);
        }
    }
}
