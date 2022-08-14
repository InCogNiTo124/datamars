use crate::ops::Operator;

pub struct Sum {
    result: f64,
}

impl Operator for Sum {
    fn apply(&mut self, x: f64) {
        self.result += x;
    }

    fn result(&self) -> f64 {
        self.result
    }
}
impl Sum {
    pub const fn new() -> Self {
        Self { result: 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Operator, Sum};

    #[test]
    fn test1() {
        let mut obj = Sum::new();
        let test_cases = vec![
            (1, 1.0),
            (2, 3.0),
            (3, 6.0),
            (4, 10.0),
            (5, 15.0),
            (6, 21.0),
            (7, 28.0),
            (8, 36.0),
            (9, 45.0),
            (100, 145.0),
        ];
        for (x, y) in test_cases {
            obj.apply(x as f64);
            let error = (obj.result() - y).abs();
            assert!(error < 1e-15);
        }
    }
}
