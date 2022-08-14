use crate::ops::Operator;

pub struct Max {
    result: f64,
}

impl Operator for Max {
    fn apply(&mut self, x: f64) {
        if x > self.result {
            self.result = x;
        }
    }

    fn result(&self) -> f64 {
        self.result
    }
}

impl Max {
    pub const fn new() -> Self {
        Self {
            result: f64::NEG_INFINITY,
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{Max, Operator};

    #[test]
    fn test1() {
        let mut obj = Max::new();
        let test_cases = vec![
            (1, 1.0),
            (2, 2.0),
            (3, 3.0),
            (4, 4.0),
            (5, 5.0),
            (6, 6.0),
            (7, 7.0),
            (8, 8.0),
            (9, 9.0),
            (100, 100.0),
        ];
        for (x, y) in test_cases {
            obj.apply(f64::from(x));
            let error = (obj.result() - y).abs();
            assert!(error < 1e-15);
        }
    }
}
