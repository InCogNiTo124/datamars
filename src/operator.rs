pub trait Operator {
    fn apply(&mut self, x: f64);
    fn result(&self) -> f64;
}