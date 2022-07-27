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
