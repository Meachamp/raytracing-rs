use crate::ray::*;
use crate::hittable::*;
use crate::vec3::*;

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}
