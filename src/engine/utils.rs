use std::f64;

// constants
pub const INFINITY: f64 = f64::MAX;
pub const PI: f64 = f64::consts::PI;

// utility functions

/// convert value from degrees to radians
pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
