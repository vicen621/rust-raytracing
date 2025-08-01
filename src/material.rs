use crate::{hittable::HitRecord, ray::Ray, vec3::{self, Color, Vec3}};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector().normalize();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray::new(hit_record.point, scatter_direction);

        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray.direction().reflect(hit_record.normal).normalize();
        let scattered = Ray::new(hit_record.point, reflected);

        if vec3::dot(scattered.direction(), hit_record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
