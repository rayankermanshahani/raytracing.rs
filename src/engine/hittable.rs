// src/engine/hittable.rs

use crate::engine::{
    ray::Ray,
    vec3::{self, Point3, Vec3},
};

pub struct HitRecord {
    p: Point3,        // 3d point where ray intersects surface
    normal: Vec3,     // surface normal at intersection point
    t: f64,           // parametric distance along the ray where intersection occurs
    front_face: bool, // flag indicating whether ray hit from outside (true) or inside (false)
}

impl HitRecord {}

impl HitRecord {
    /// creates a new empty HitRecord
    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
        }
    }

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
    /// returns a HitRecord's front face
    pub fn front_face(&self) -> bool {
        self.front_face
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

    /// setter for HitRecord's front face
    pub fn set_front_face(&mut self, front_face: bool) {
        self.front_face = front_face;
    }

    /// sets the hit record's normal vector
    /// NOTE: outward_normal parameter is assumed to have unit length
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        // determine if ray hit from outside (front face)
        self.set_front_face(vec3::dot(&ray.direction(), &outward_normal) < 0.0);

        // ensure surface normal always points against the ray
        self.set_normal(if self.front_face() {
            *outward_normal // ray is inside sphere
        } else {
            -(*outward_normal) // ray is outside sphere
        });
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}
