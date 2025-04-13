#![allow(dead_code)]
mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use color::Color;
use ray::Ray;
use std::io::{self, Write};
use vec3::{Point3, Vec3, dot};

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0; // compute height via aspect ratio
    let image_width = 1440;
    let ar_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = if ar_height < 1 { 1 } else { ar_height };

    // camera and viewport (dimensions are real valued)
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = vec3::Point3::zero();

    // calculate vectors across the horizontal and down the vertical
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // calculate horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // calculate location of the upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r);
            _ = color::write_color(&mut io::stdout(), pixel_color);
        }
    }
    eprint!("\rDone.                       \n");
}

// ray-sphere intersection
fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc: Vec3 = *center - ray.origin();
    let a = ray.direction().length_squared();
    let h = dot(&ray.direction(), &oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        // only using the closest (smallest t) hitpoint for now
        return (h - discriminant.sqrt()) / a;
    }
}

fn ray_color(ray: &Ray) -> Color {
    // define the sphere
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        // find the surface normal (unit vector)
        let n = vec3::unit_vector(ray.at(t) - Vec3::new(0.0, 0.0, -1.0));
        // each component of surface normal coresponds to a color (x: red, y: green, z: blue)
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = vec3::unit_vector(ray.direction());
    // transform unit vector's range from [-1.0,1.0] to [0.0,1.0]
    let a = 0.5 * (unit_direction.y() + 1.0);
    // background color is a gradient from blue (top) to white (bottom)
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}
