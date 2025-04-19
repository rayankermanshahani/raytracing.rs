// src/engine/interval.rs

use crate::engine::utils::INFINITY;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    /// creates a new interval of given values
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min: min, max: max }
    }

    /// creates an interval of no values
    pub fn empty() -> Interval {
        Interval {
            min: INFINITY,
            max: -INFINITY,
        }
    }

    /// creates an interval of every possible value
    pub fn universe() -> Interval {
        Interval {
            min: -INFINITY,
            max: INFINITY,
        }
    }

    /// returns the size of an interval
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// returns whether a value is contained in an interval
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// returns whether a value is surrounded by an interval
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval::empty()
    }
}
