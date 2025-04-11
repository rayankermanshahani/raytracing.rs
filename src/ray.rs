use crate::vec3::{Point3, Vec3};

/// a semi-infinite line
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    /// constructs new `Ray` from given origin and direction
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    /// returns ray's origin point
    pub fn origin(&self) -> Point3 {
        self.origin
    }

    /// returns ray's direction vector
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    /// returns the 3d point on the 2d ray at parameter t
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray {
            origin: Point3::zero(),
            direction: Point3::zero(),
        }
    }
}
