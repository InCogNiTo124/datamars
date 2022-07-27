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
