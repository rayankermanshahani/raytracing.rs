// src/hittable.rs

use crate::engine::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
}

impl HitRecord {
    /// returns a HitRecord's point p
    pub fn p(&self) -> Point3 {
        self.p
    }

    /// returns a HitRecord's surface normal vector
    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    /// returns a HitRecord's t parameter
    pub fn t(&self) -> f64 {
        self.t
    }

    /// setter for HitRecord's point p
    pub fn set_p(&mut self, p: Point3) {
        self.p = p;
    }
    /// setter for HitRecord's surface normal vector
    pub fn set_normal(&mut self, normal: Vec3) {
        self.normal = normal;
    }
    /// setter for HitRecord's t parameter
    pub fn set_t(&mut self, t: f64) {
        self.t = t;
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}
