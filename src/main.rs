// src/main.rs

#![allow(dead_code)]

use raytracing_rs::engine::{
    camera::Camera, hittable_list::HittableList, sphere::Sphere, vec3::Point3,
};
use std::rc::Rc;

fn main() {
    // create world
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5))); // gradient "sky"
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0))); // green grass

    // create camera
    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 2560;
    cam.samples_per_pixel = 100;

    cam.render(&world);
}
