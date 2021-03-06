use crate::*;
use vec3::Vec3;
use ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, aspect_ratio: f64, vfov: f64, aperture: f64, focus_dist: f64) -> Self {
        //let aspect_ratio = 16.0/9.0;

        let theta = util::deg_to_rad(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0*h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit(&(look_from - look_at));
        let u = Vec3::unit(&Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);

        let origin = look_from;
        let horizontal = focus_dist*viewport_width*u;
        let vertical = focus_dist*viewport_height*v;
        let lower_left = origin - horizontal/2.0 - vertical/2.0 - focus_dist*w;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left,
            u,
            v,
            lens_radius: aperture/2.0
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius*util::random_unit_disk();
        let offset = self.u*rd.x() + self.v*rd.y();

        let v = self.lower_left + u*self.horizontal + v*self.vertical - self.origin - offset;
        Ray::new(&(self.origin + offset), &v)
    }
}
