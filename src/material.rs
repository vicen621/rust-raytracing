use crate::{
    common,
    hittable::HitRecord,
    ray::Ray,
    vec3::{self, Color, Vec3},
};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)>;
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
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit.normal; // If the scatter direction is near zero, use the normal
        }

        let scattered = Ray::new(hit.point, scatter_direction);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let mut reflected = ray.direction().reflect(hit.normal);
        reflected = reflected.normalize() + (Vec3::random_unit_vector() * self.fuzz);
        let scattered = Ray::new(hit.point, reflected);

        if vec3::dot(scattered.direction(), hit.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None // The ray is not scattered if it reflects back into the surface
        }
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction().normalize();

        let cos_theta = vec3::dot(-unit_direction, hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let will_reflect = Dielectric::reflectance(cos_theta, refraction_ratio) > common::random_double();

        let direction = if cannot_refract || will_reflect {
            unit_direction.reflect(hit.normal)
        } else {
            unit_direction.refract(hit.normal, refraction_ratio)
        };

        let scattered = Ray::new(hit.point, direction);
        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}
