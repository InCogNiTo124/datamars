pub use crate::operator::operator;

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
    pub fn new() -> Sum {
        Sum { result: 0.0 }
    }
}
