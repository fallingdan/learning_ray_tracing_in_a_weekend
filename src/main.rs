use std::io;

use indicatif::{ProgressBar, ProgressStyle};
use vec3::{Point3, Vec3};

use crate::color::Color;
use crate::ray::Ray;
mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;

    // Calculate image height and make sure its at least 1
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // Camera
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 =
        viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical
    // viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the Horizontal and Vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // calculate the location of the upper left pixel
    let viewport_upper_left = camera_center
        - Vec3::new(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;

    let pixel00_loc =
        viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    let style = ProgressStyle::with_template("{msg}").unwrap();
    let progress_bar: ProgressBar =
        ProgressBar::new(image_height.into()).with_style(style);

    let mut stdout = io::stdout();

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel00_loc
                + (i as f64 * pixel_delta_u)
                + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray);
            color::write_color(&mut stdout, &pixel_color);
        }
        progress_bar.inc(1);
    }

    progress_bar.finish_with_message("\nDone\n");
}

fn ray_color(ray: &Ray) -> Color {
    let sphere = Point3::new(0.0, 0.0, -1.0);
    let t = hit_sphere(&sphere, 0.5, ray);

    if t > 0.0 {
        let normal = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).unitvector();
        return 0.5
            * Color::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
    }

    let unit_direction = Vec3::unitvector(ray.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = *center - ray.origin();
    let a = Vec3::dotproduct(&ray.direction(), &ray.direction());
    let b = -2.0 * Vec3::dotproduct(&ray.direction(), &oc);
    let c = Vec3::dotproduct(&oc, &oc) - (radius * radius);
    let discriminant = (b * b) - (4.0 * a * c);

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}
