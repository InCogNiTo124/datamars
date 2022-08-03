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
        Self { result: f64::NEG_INFINITY }
    }
}
