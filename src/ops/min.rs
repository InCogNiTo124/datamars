use crate::ops::Operator;

pub struct Min {
    result: f64,
}

impl Operator for Min {
    fn apply(&mut self, x: f64) {
        if x < self.result {
            self.result = x;
        }
    }

    fn result(&self) -> f64 {
        self.result
    }
}

impl Min {
    pub const fn new() -> Self {
        Self {
            result: f64::INFINITY,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Min, Operator};

    #[test]
    fn test1() {
        let mut obj = Min::new();
        let test_cases = vec![
            (1, 1.0),
            (2, 1.0),
            (3, 1.0),
            (4, 1.0),
            (5, 1.0),
            (6, 1.0),
            (7, 1.0),
            (8, 1.0),
            (9, 1.0),
            (100, 1.0),
        ];
        for (x, y) in test_cases {
            obj.apply(f64::from(x));
            let error = (obj.result() - y).abs();
            assert!(error < 1e-15);
        }
    }
}
