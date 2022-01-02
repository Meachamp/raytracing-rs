use crate::*;
use vec3::Vec3;
use ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0/9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vec3::from_f64(0.0, 0.0, 0.0);
        let horizontal = Vec3::from_f64(viewport_width, 0.0, 0.0);
        let vertical = Vec3::from_f64(0.0, viewport_height, 0.0);
        let lower_left = origin - horizontal/2.0 - vertical/2.0 - Vec3::from_f64(0.0, 0.0, focal_length);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let v = self.lower_left + u*self.horizontal + v*self.vertical - self.origin;
        Ray::new(&self.origin, &v)
    }
}