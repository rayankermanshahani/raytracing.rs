// src/engine/hittable_list.rs

use crate::engine::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

use std::rc::Rc;

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

    /// creates a new HittableList with a single object
    pub fn with_object(object: Rc<dyn Hittable>) -> HittableList {
        let mut list = HittableList::new();
        list.add(object);
        list
    }

    /// clears all objects from list
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    /// adds object to the list
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut closest_so_far = ray_tmax;
        let mut hit_anything = false;

        for object in &self.objects {
            if object.hit(ray, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t();

                // copy fields from temp_rec to rec
                rec.set_p(temp_rec.p());
                rec.set_normal(temp_rec.normal());
                rec.set_t(temp_rec.t());
                rec.set_front_face(temp_rec.front_face());
            }
        }

        hit_anything
    }
}
