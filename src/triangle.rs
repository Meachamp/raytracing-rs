use crate::vec3::*;
use crate::ray::*;
use crate::hittable::*;
use crate::material::Material;
use std::sync::Arc;
use crate::aabb::*;

pub struct Triangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    n: Vec3,
    material: Arc<dyn Material>
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material: Arc<dyn Material>) -> Self {
        let e10 = v1-v0;
        let e20 = v2-v0;
        let c = Vec3::cross(&e10, &e20);

        let n = Vec3::unit(&c);

        Self {
            v0,
            v1,
            v2,
            n,
            material
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: Ray, min: f64, max: f64, hit_record: &mut HitRecord) -> bool {
        let dir = ray.direction();
        let n = self.n;

        let n_dot_dir = Vec3::dot(&n, &dir);
        if n_dot_dir.abs() < 1e-8 {
            return false; //Ray is parallel to the triangle
        }

        //if n_dot_dir < 0.0 {
        //    return false;
        //}

        let origin = ray.origin();

        let t = Vec3::dot(&(self.v0-origin), &n) / Vec3::dot(&n, &dir);

        if t < min || t > max {
            return false;
        }
        //println!("{}", t);
        let p = ray.at(t);

        let e10 = self.v1 - self.v0;
        let ep0 = p - self.v0;
        let c = Vec3::cross(&e10, &ep0);
        let a = Vec3::dot(&c, &n);
        if a < 0.0 { return false; }

        let e20 = self.v2 - self.v0;
        let c = Vec3::cross(&ep0, &e20);
        let a = Vec3::dot(&c, &n);
        if a < 0.0 { return false; }

        let e21 = self.v2 - self.v1;
        let ep1 = p - self.v1;
        let c = Vec3::cross(&e21, &ep1);
        let a = Vec3::dot(&c, &n);
        if a < 0.0 { return false; }

        hit_record.p = p;
        hit_record.t = t;
        hit_record.set_face_normal(&ray, &n);
        hit_record.material = self.material.clone();

        return true;
    }

    fn bounding_box(&self) -> Option<AABB> {
        let pts = vec![self.v0, self.v1, self.v2];
        let x_min = pts.iter().map(|v| v[0]).reduce(f64::min).unwrap();
        let y_min = pts.iter().map(|v| v[1]).reduce(f64::min).unwrap();
        let z_min = pts.iter().map(|v| v[2]).reduce(f64::min).unwrap();

        let x_max = pts.iter().map(|v| v[0]).reduce(f64::max).unwrap();
        let y_max = pts.iter().map(|v| v[1]).reduce(f64::max).unwrap();
        let z_max = pts.iter().map(|v| v[2]).reduce(f64::max).unwrap();

        Some(AABB::new(Vec3::from_f64(x_min, y_min, z_min), Vec3::from_f64(x_max, y_max, z_max)))
    }
}
