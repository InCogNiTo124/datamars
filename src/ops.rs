mod geomean;
mod harmean;
mod mean;
mod median;
mod std;
mod sum;
mod min;
mod max;

pub use crate::ops::std::Std;
pub use geomean::GeoMean;
pub use harmean::HarMean;
pub use mean::Mean;
pub use median::Median;
pub use sum::Sum;
pub use min::Min;
pub use max::Max;

pub trait Operator {
    fn apply(&mut self, x: f64);
    fn result(&self) -> f64;
}
