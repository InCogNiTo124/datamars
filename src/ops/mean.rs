use crate::ops::Operator;

pub struct Mean {
    pub(in crate::ops) n: i64,
    result: f64,
}

impl Operator for Mean {
    fn apply(&mut self, x: f64) {
        self.n += 1;
        self.result += (x - self.result) / (self.n as f64);
    }

    fn result(&self) -> f64 {
        self.result
    }
}

impl Mean {
    pub const fn new() -> Self {
        Self { n: 0, result: 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Mean, Operator};

    #[test]
    fn test1() {
        let mut obj = Mean::new();
        let test_cases = vec![
            (1, 1.0),
            (2, 1.5),
            (3, 2.0),
            (4, 2.5),
            (5, 3.0),
            (6, 3.5),
            (7, 4.0),
            (8, 4.5),
            (9, 5.0),
            (100, 14.5),
        ];
        for (x, y) in test_cases {
            obj.apply(x as f64);
            assert!((obj.result() - y).abs() < 1e-7);
        }
    }
}
