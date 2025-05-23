// src/engine/sphere.rs

use crate::engine::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::{self, Point3},
};

use super::interval::Interval;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    /// constructs new `Sphere` from given center point and radius length
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius: radius.max(0.0),
        }
    }

    /// returns the sphere's center point
    pub fn center(&self) -> Point3 {
        self.center
    }

    /// returns the sphere's radius
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// sets the sphere's center point
    pub fn set_center(&mut self, center: Point3) {
        self.center = center;
    }

    /// sets the sphere's radius
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = vec3::dot(&ray.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }
        let sqrt_discriminant = discriminant.sqrt();

        // find nearest root that lies in acceptable range
        let root1 = (h - sqrt_discriminant) / a;
        if ray_t.contains(root1) {
            // check smaller root first
            rec.set_t(root1);
            rec.set_p(ray.at(rec.t()));
            let outward_normal = (rec.p() - self.center()) / self.radius();
            rec.set_face_normal(ray, &outward_normal);
            return true;
        }

        // if smaller root is invalid, check larger root
        let root2 = (h + sqrt_discriminant) / a;
        if ray_t.contains(root2) {
            rec.set_t(root2);
            rec.set_p(ray.at(rec.t()));
            let outward_normal = (rec.p() - self.center()) / self.radius();
            rec.set_face_normal(ray, &outward_normal);
            return true;
        }

        // neither root is in valid interval
        return false;
    }
}
