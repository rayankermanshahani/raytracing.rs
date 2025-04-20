use rand::Rng;
use std::f64;

// constants
pub const INFINITY: f64 = f64::MAX;
pub const PI: f64 = f64::consts::PI;

// utility functions

/// convert value from degrees to radians
pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// returns a random real number in [0,1)
pub fn random_0_to_1() -> f64 {
    rand::rng().random()
}

/// returns a random real number in [0,1)
pub fn random_min_to_max(min: f64, max: f64) -> f64 {
    rand::rng().random_range(min..max)
}
