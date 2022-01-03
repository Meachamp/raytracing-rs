use rand::{thread_rng, Rng, distributions::Uniform};
use crate::vec3::Vec3;
use std::f64::consts::PI;

pub fn random_double() -> f64 {
    thread_rng().gen()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    let between = Uniform::new(min, max);
    thread_rng().sample(between)
}

pub fn refract(uv: &Vec3, n: &Vec3, k: f64) -> Vec3 {
    let cos_theta = Vec3::dot(&-(*uv), n);
    let cos_theta = f64::min(cos_theta, 1.0);

    let r_perp = k*(*uv + cos_theta*(*n));
    let r_para = -((1.0 - r_perp.length_squared()).abs().sqrt()) * (*n);

    r_perp + r_para
}

pub fn deg_to_rad(deg: f64) -> f64 {
    deg / 180.0 * PI
}
