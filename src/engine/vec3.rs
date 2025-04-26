// src/engine/vec3.rs

use std::fmt;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::engine::utils::{random_0_to_1, random_min_to_max};

/// a three-dimensional vector of f64
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    /// constructs new `Vec3` from given components
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    /// constructs new `Vec3` with all components initialized to zero
    pub fn zero() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    /// constructs a new `Vec3` with random valued components
    pub fn random() -> Vec3 {
        Vec3 {
            e: [random_0_to_1(), random_0_to_1(), random_0_to_1()],
        }
    }

    /// constructs a new `Vec3` with random valued components within a specified range
    pub fn random_in_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            e: [
                random_min_to_max(min, max),
                random_min_to_max(min, max),
                random_min_to_max(min, max),
            ],
        }
    }

    /// returns x component
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    // returns y component
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    /// returns z component
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    /// sets x component
    pub fn set_x(&mut self, x: f64) {
        self.e[0] = x;
    }

    /// sets y component
    pub fn set_y(&mut self, y: f64) {
        self.e[1] = y;
    }
    /// sets z component
    pub fn set_z(&mut self, z: f64) {
        self.e[2] = z;
    }

    /// returns the squared length of the vector
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    /// returns the length (magnitude of the vector)
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

// `Point3` is just an alias for `Vec3`
pub type Point3 = Vec3;

// vector negation: -Vec3
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

// allow indexing into `Vec3` using [i] operator
impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

// elementwise addition: Vec3 + Vec3
impl Add for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

// elementwise addition assignment: Vec3 += Vec3
impl AddAssign for Vec3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

// elementwise subtraction: Vec3 - Vec3
impl Sub for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Vec3 {
        Vec3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

// elementwise subtraction assignment: Vec3 -= Vec3
impl SubAssign for Vec3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];
    }
}

// element-wise multiplication: Vec3 * Vec3
impl Mul for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}

// scalar multiplication: Vec3 * f64
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, scalar: f64) -> Vec3 {
        Vec3::new(self.e[0] * scalar, self.e[1] * scalar, self.e[2] * scalar)
    }
}

// scalar multiplication: f64 * Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

// scalar multiplication assignment: Vec3 *= f64
impl MulAssign<f64> for Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

// scalar division: Vec3 / f64
impl Div<f64> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn div(self, rhs: f64) -> Vec3 {
        self * (1.0 / rhs)
    }
}

// scalar division assignment: Vec3 /= f64
impl DivAssign<f64> for Vec3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

// pretty printing using `{}` formatter
impl fmt::Display for Vec3 {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::zero()
    }
}

/// returns the dot product of two `Vec3` instances
#[inline(always)]
pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

/// returns the cross product of two `Vec3` instances
#[inline(always)]
pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

/// returns the (normalized) unit vector of a given `Vec3`
#[inline(always)]
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

/// creates and returns a random unit vector
#[inline(always)]
pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::random_in_range(-1.0, 1.0);
        let lensq = p.length_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}
