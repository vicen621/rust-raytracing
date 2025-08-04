use rayon::prelude::*;

use crate::{
    common,
    hittable::Hittable,
    hittable_list::HittableList,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

pub struct Camera {
    image_width: u64,
    image_height: u64,
    center: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    upper_left_corner: Point3,
    samples_per_pixel: u64,
    max_depth: u64,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u64,
        samples_per_pixel: u64,
        max_depth: u64,
    ) -> Self {
        let image_height: u64 = (image_width as f64 / aspect_ratio) as u64;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let center = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, -viewport_height, 0.0);
        let upper_left_corner =
            center - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            image_width,
            image_height,
            center,
            horizontal,
            vertical,
            upper_left_corner,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn render(&self, world: &HittableList) {
        println!("P3");
        println!("{} {} ", self.image_width, self.image_height);
        println!("255"); // Max color value);

        for j in 0..self.image_height {
            // Progress report
            eprint!("\rScanlines done: {}/{}", j + 1, self.image_height);

            let pixel_colors: Vec<_> = (0..self.image_width)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                    for _ in 0..self.samples_per_pixel {
                        let r = self.get_ray(i, j);
                        pixel_color += Camera::ray_color(&r, world, self.max_depth);
                    }

                    pixel_color
                })
                .collect();

            for pixel_color in pixel_colors {
                println!("{}", pixel_color.format_color(self.samples_per_pixel));
            }
        }

        eprint!("\nDone.\n");
    }

    fn get_ray(&self, i: u64, j: u64) -> Ray {
        let delta_horizontal = (i as f64 + common::random_double()) / (self.image_width - 1) as f64;
        let delta_vertical = (j as f64 + common::random_double()) / (self.image_height - 1) as f64;

        Ray::new(
            self.center,
            self.upper_left_corner
                + delta_horizontal * self.horizontal
                + delta_vertical * self.vertical
                - self.center,
        )
    }

    fn ray_color(r: &Ray, world: &impl Hittable, depth: u64) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0); // Return black if depth is zero
        }

        if let Some(hit) = world.hit(r, 0.001, common::INFINITY) {
            if let Some((attenuation, scattered)) = hit.material.scatter(r, &hit) {
                return attenuation * Camera::ray_color(&scattered, world, depth - 1);
            }

            return Color::default(); // Return black if no scattering occurs
        }

        let unit_direction = r.direction().normalize();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
