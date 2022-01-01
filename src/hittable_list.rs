use std::rc::Rc;
use crate::*;
use hittable::*;

pub struct HittableList {
    list: Vec<Rc<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            list: Vec::new()
        }
    }

    pub fn add(&mut self, h: Rc<dyn Hittable>) {
        self.list.push(h);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, min: f64, max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp = HitRecord::new();
        let mut hit = false;
        let mut closest = max;

        for obj in &self.list {
            if obj.hit(r, min, closest, &mut temp) {
                hit = true;
                closest = temp.t;
                *hit_record = temp;
            }
        }

        hit
    }
}
