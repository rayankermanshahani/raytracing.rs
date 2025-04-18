// src/engine/hittable_list.rs

use crate::engine::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

use std::rc::Rc;

use super::interval::Interval;

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    /// creates a new empty HittableList
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    /// adds object to the list
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    /// clears all objects from list
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(ray, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t();
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
