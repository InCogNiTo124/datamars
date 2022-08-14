mod geomean;
mod harmean;
mod max;
mod mean;
mod median;
mod min;
mod std;
mod sum;

pub use crate::ops::std::Std;
pub use geomean::GeoMean;
pub use harmean::HarMean;
pub use max::Max;
pub use mean::Mean;
pub use median::Median;
pub use min::Min;
pub use sum::Sum;

pub trait Operator {
    fn apply(&mut self, x: f64);
    fn result(&self) -> f64;
}
