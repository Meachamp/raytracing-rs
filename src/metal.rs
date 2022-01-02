use crate::*;
use vec3::*;
use ray::*;
use material::Material;

pub struct Metal {
    albedo: Vec3
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let dir = Vec3::reflect(&Vec3::unit(&ray.direction()), &rec.normal);

        *scattered = Ray::new(&rec.p, &dir);
        *attenuation = self.albedo;

        Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
}


impl Metal {
    pub fn new(vec: Vec3) -> Self {
        Self {
            albedo: vec
        }
    }
}
