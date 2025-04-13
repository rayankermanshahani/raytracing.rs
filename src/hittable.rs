// src/hittable.rs

use crate::{
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
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}
