// src/engine/camera.rs

use crate::engine::{
    color::{self, Color},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    utils::{self, INFINITY},
    vec3::{self, Point3, Vec3},
};
use std::io::{self, Write};

#[derive(Default, Clone, Copy)]
pub struct Camera {
    // private camera parameters
    image_height: i32,        // rendered image height in pixel count
    center: Point3,           // camera center
    pixel00_loc: Point3,      // location of pixel 0,0
    pixel_delta_u: Vec3,      // offset to pixel to the right
    pixel_delta_v: Vec3,      // offset to pixel below
    pixel_samples_scale: f64, // color scale factor for a sum of pixels

    // public camera parameters
    pub aspect_ratio: f64,      // ratio of image width over height
    pub image_width: i32,       // rendered image width in pixel count
    pub samples_per_pixel: i32, // count of random samples for each pixel
}

impl Camera {
    // private camera functions
    fn initialize(&mut self) {
        let ar_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if ar_height < 1 { 1 } else { ar_height };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = Point3::zero();

        // determine viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // calculate vectors across horizontal and down vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // calculate horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // calculate the location of the upper left pixel
        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(&self, ray: &Ray, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::new();
        if world.hit(&ray, Interval::new(0.0, INFINITY), &mut rec) {
            return 0.5 * (rec.normal() + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = vec3::unit_vector(ray.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    /// construct a camera ray originating from the origin and directed at randomly sampled piont around the pixel location i,j
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    /// returns the vector to a random point in the [-0.5,-0.5] to [0.5,0.5] unit square
    fn sample_square(&self) -> Vec3 {
        Vec3::new(
            utils::random_0_to_1() - 0.5,
            utils::random_0_to_1() - 0.5,
            0.0,
        )
    }

    // public camera functions
    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        // header for ppm file
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {}", self.image_height - j);
            io::stderr().flush().unwrap();
            for i in 0..self.image_width {
                let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, world);
                }
                _ = color::write_color(&mut io::stdout(), self.pixel_samples_scale * pixel_color);
            }
        }
        eprint!("\rDone.                      \n");
    }
}
