pub mod operator {
    pub trait Operator {
        fn new() -> Self;
        fn apply(&mut self, x: f64);
        fn result(&self) -> f64;
    }
}
