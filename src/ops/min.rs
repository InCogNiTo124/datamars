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
        Self { result: f64::INFINITY }
    }
}
