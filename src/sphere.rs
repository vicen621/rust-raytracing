use std::rc::Rc;

use crate::{hittable::{HitRecord, Hittable}, material::Material, ray::Ray, vec3::{self, Point3}};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Sphere { center, radius, material }
    }

    pub fn center(&self) -> Point3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root <= t_min || t_max <= root {
            root = (-half_b + sqrtd) / a;
            if root <= t_min || t_max <= root {
                return None;
            }
        }

        let outward_normal = (r.at(root) - self.center) / self.radius;
        Some(HitRecord::new(r, root, outward_normal, self.material.clone()))
    }
}
