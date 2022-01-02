use crate::*;
use vec3::*;
use ray::*;
use material::Material;

pub struct Lambertian {
    albedo: Vec3
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut dir = rec.normal + Vec3::random_unit_vector();

        if dir.near_zero() {
            dir = rec.normal;
        }

        *scattered = Ray::new(&rec.p, &dir);
        *attenuation = self.albedo;

        true
    }
}

impl Lambertian {
    pub fn new(vec: Vec3) -> Self {
        Self {
            albedo: vec
        }
    }
}
