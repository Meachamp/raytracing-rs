use crate::vec3::*;

#[derive(Copy, Clone)]
pub struct Ray {
    dir: Vec3,
    origin: Point3
}

impl Ray {
    pub fn new(origin: &Point3, dir: &Vec3) -> Self {
        Self {
            dir: *dir,
            origin: *origin
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t*self.dir
    }
}
