// constants
pub use std::f64::consts::PI;
pub use std::f64::INFINITY;
use std::ops::RangeInclusive;

use rand::Rng;

// utility functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    // Return a random real in [0.0, 1.0)
    rand::rng().random()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    // Return a random real in [min, max)
    min + (max - min) * random_double()
}

pub fn clamp(x: f64, range: RangeInclusive<f64>) -> f64 {
    if &x < range.start() {
        range.start().clone()
    } else if &x > range.end() {
        range.end().clone()
    } else {
        x
    }
}
