use crate::*;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Vec3::new(),
            normal: Vec3::new(),
            t: 0.0,
            front_face: true
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&ray.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, min: f64, max: f64, hit_record: &mut HitRecord) -> bool;
}
