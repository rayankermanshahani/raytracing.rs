use rand::Rng;
use std::cell::RefCell;
use std::f64;

thread_local! {
    static RNG: RefCell<rand::rngs::ThreadRng> = RefCell::new(rand::rng());
}

// constants
pub const INFINITY: f64 = f64::MAX;
pub const PI: f64 = f64::consts::PI;

// utility functions

/// convert value from degrees to radians
#[inline(always)]
pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// returns a random real number in [0,1)
#[inline(always)]
pub fn random_0_to_1() -> f64 {
    RNG.with(|rng| {
        let mut rng = rng.borrow_mut();
        rng.random_range(0.0..1.0)
    })
}

/// returns a random real number in [0,1)
#[inline(always)]
pub fn random_min_to_max(min: f64, max: f64) -> f64 {
    RNG.with(|rng| {
        let mut rng = rng.borrow_mut();
        rng.random_range(min..max)
    })
}
