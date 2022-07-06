use crate::operator;

pub struct Sum {
    result: f64,
}

impl operator::Operator for Sum {
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
