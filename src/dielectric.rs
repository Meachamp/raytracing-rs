use crate::*;
use vec3::*;
use ray::*;
use material::Material;

pub struct Dieletric {
    ir: f64
}

impl Material for Dieletric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        *attenuation = Vec3::from_f64(1.0, 1.0, 1.0);

        let refract_ratio = if rec.front_face { 1.0/self.ir } else {self.ir};
        let unit_dir = Vec3::unit(&ray.direction());

        let cos_theta = Vec3::dot(&-unit_dir, &rec.normal);
        let sin_theta = (1.0-cos_theta*cos_theta).sqrt();

        let should_reflect = refract_ratio * sin_theta > 1.0;


        let direction = if should_reflect || Dieletric::reflectance(cos_theta, refract_ratio) > util::random_double() {
            Vec3::reflect(&unit_dir, &rec.normal)
        } else {
            util::refract(&unit_dir, &rec.normal, refract_ratio)
        };

        *scattered = Ray::new(&rec.p, &direction);
        true
    }
}


impl Dieletric {
    pub fn new(ir: f64) -> Self {
        Self {
            ir: ir
        }
    }

    fn reflectance(cosine: f64, k: f64) -> f64 {
        let r0 = (1.0-k)/(1.0+k);
        let r0 = r0*r0;

        r0 + (1.0-r0)*(1.0-cosine).powf(5.0)
    }
}
