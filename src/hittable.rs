use std::{sync::Arc};

use crate::{material::Material, ray::Ray, vec3::{self, Point3, Vec3}};

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material: Arc<dyn Material>,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, root: f64, outward_normal: Vec3, material: Arc<dyn Material>) -> Self {
        let point = ray.at(root);
        let front_face = vec3::dot(ray.direction(), outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            point,
            normal,
            t: root,
            material,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
