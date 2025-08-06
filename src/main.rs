use std::sync::Arc;

use raytracer::{
    camera::Camera,
    common,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Material, Metal},
    sphere::Sphere,
    vec3::{Color, Point3, Vec3},
};

fn main() {
    // World
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = common::random_double();
            let center = Point3::new(
                a as f64 + 0.9 * common::random_double(),
                0.2,
                b as f64 + 0.9 * common::random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Arc<dyn Material> = if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();
                    Arc::new(Lambertian::new(albedo))
                } else if choose_material < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = common::random_double_range(0.0, 0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    Arc::new(Dielectric::new(1.5))
                };

                world.add(Box::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 1200;
    const SAMPLES_PER_PIXEL: u64 = 500;
    const MAX_DEPTH: u64 = 50;
    const VERTICAL_FOV: f64 = 20.0;
    const LOOK_FROM: Point3 = Point3::new(13.0, 2.0, 3.0);
    const LOOK_AT: Point3 = Point3::new(0.0, 0.0, 0.0);
    const VIEW_UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    const APERTURE: f64 = 0.1;
    let focus_distance: f64 = 10.0;

    let camera = Camera::new(
        ASPECT_RATIO,
        IMAGE_WIDTH,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        VERTICAL_FOV,
        LOOK_FROM,
        LOOK_AT,
        VIEW_UP,
        APERTURE,
        focus_distance,
    );

    camera.render(&world);
}
