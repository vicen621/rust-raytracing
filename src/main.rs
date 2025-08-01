use std::rc::Rc;

use raytracer::{camera::Camera, hittable_list::HittableList, material::{Lambertian, Metal}, sphere::Sphere, vec3::{Color, Point3}};

fn main() {
    // World
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left   = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right  = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    let ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground);
    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center);
    let sphere_left   = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left);
    let sphere_right  = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right);

    world.add(Box::new(ground));
    world.add(Box::new(sphere_center));
    world.add(Box::new(sphere_left));
    world.add(Box::new(sphere_right));

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 400;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 50;

    let camera = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, SAMPLES_PER_PIXEL, MAX_DEPTH);

    camera.render(&world);
}
