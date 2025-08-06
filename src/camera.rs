use rayon::prelude::*;

use crate::{
    common,
    hittable::Hittable,
    hittable_list::HittableList,
    ray::Ray,
    vec3::{cross, Color, Point3, Vec3},
};

pub struct Camera {
    image_width: u64,
    image_height: u64,
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    upper_left_corner: Point3,
    samples_per_pixel: u64,
    max_depth: u64,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u64,
        samples_per_pixel: u64,
        max_depth: u64,
        vertical_fov: f64,
        look_from: Point3,
        look_at: Point3,
        view_up: Vec3,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let image_height: u64 = (image_width as f64 / aspect_ratio) as u64;
        let theta = common::degrees_to_radians(vertical_fov);
        let half_height = (theta / 2.0).tan();
        let viewport_height = 2.0 * half_height;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (look_from - look_at).normalize();
        let u = cross(view_up, w).normalize();
        let v = cross(w, u);

        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * -v;
        let upper_left_corner = origin - (focus_distance * w) - horizontal / 2.0 - vertical / 2.0;

        let lens_radius = aperture / 2.0;

        Camera {
            image_width,
            image_height,
            origin,
            horizontal,
            vertical,
            upper_left_corner,
            samples_per_pixel,
            max_depth,
            lens_radius,
            u,
            v
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
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.

        let delta_horizontal = (i as f64 + common::random_double()) / (self.image_width - 1) as f64;
        let delta_vertical = (j as f64 + common::random_double()) / (self.image_height - 1) as f64;

        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.upper_left_corner
                + delta_horizontal * self.horizontal
                + delta_vertical * self.vertical
                - self.origin - offset,
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
