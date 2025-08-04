use std::sync::Arc;

use raytracer::{camera::Camera, hittable_list::HittableList, material::{Dielectric, Lambertian, Metal}, sphere::Sphere, vec3::{Color, Point3}};

fn main() {
    // World
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground);
    let center = Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center);
    let left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left);
    let right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right);

    world.add(Box::new(ground));
    world.add(Box::new(center));
    world.add(Box::new(left));
    world.add(Box::new(right));

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 400;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 50;

    let camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, SAMPLES_PER_PIXEL, MAX_DEPTH);

    camera.render(&world);
}
