use crate::*;
use hittable::*;
use material::Material;
use std::sync::Arc;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Arc<dyn Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material: mat
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, min: f64, max: f64, hit_record: &mut HitRecord) -> bool {
        let a = Vec3::dot(&ray.direction(), &ray.direction());
        let oc = ray.origin() - self.center;
        let b = 2.0*Vec3::dot(&ray.direction(), &oc);
        let c = Vec3::dot(&oc, &oc) - self.radius*self.radius;

        let disc = b*b - 4.0*a*c;
        if disc < 0.0 {
            return false;
        }

        let s_disc = disc.sqrt();
        let mut root = (-b - s_disc) / (2.0*a);

        if root < min || root > max {
            root = (-b + s_disc) / (2.0*a);

            if root < min || root > max {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = ray.at(root);
        hit_record.material = self.material.clone();

        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(&ray, &outward_normal);

        return true;
    }
}
