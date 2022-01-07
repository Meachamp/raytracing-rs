use crate::*;
use vec3::*;
use ray::*;

#[derive(Clone, Debug)]
pub struct AABB {
    min: Vec3,
    max: Vec3
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self {
            min,
            max
        }
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.max
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        for axis in 0..3 {
            let d = 1.0 / ray.direction()[axis];
            let origin = ray.origin();

            let mut t0 = (self.min()[axis] - origin[axis]) * d;
            let mut t1 = (self.max()[axis] - origin[axis]) * d;

            if d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            let t_min = f64::max(t0, t_min);
            let t_max = f64::min(t1, t_max);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn union(box1: Option<AABB>, box2: Option<AABB>) -> Option<AABB> {
        if box2.is_none() || box1.is_none() {
            return None;
        }

        let box1 = box1.unwrap();
        let box2 = box2.unwrap();

        let box1_min = box1.min();
        let box2_min = box2.min();

        let mins = Vec3::from_f64(
                                    f64::min(box1_min[0], box2_min[0]),
                                    f64::min(box1_min[1], box2_min[1]),
                                    f64::min(box1_min[2], box2_min[2])
                                );

        let box1_max = box1.max();
        let box2_max = box2.max();

        let maxs = Vec3::from_f64(
                                    f64::max(box1_max[0], box2_max[0]),
                                    f64::max(box1_max[1], box2_max[1]),
                                    f64::max(box1_max[2], box2_max[2])
                                );

        Some(AABB::new(mins, maxs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construct() {
        let a = AABB::new(Vec3::from_f64(0.0, 0.0, 0.0), Vec3::from_f64(1.0, 1.0, 1.0));
        assert_eq!((a.min()-Vec3::from_f64(0.0, 0.0, 0.0)).length() < 1e-8, true);
        assert_eq!((a.max()-Vec3::from_f64(1.0, 1.0, 1.0)).length() < 1e-8, true);
    }

    #[test]
    fn test_hit() {
        let a = AABB::new(Vec3::from_f64(0.0, 0.0, 0.0), Vec3::from_f64(1.0, 1.0, 1.0));
        let r = Ray::new(&Vec3::from_f64(0.5, 0.5, -1.0), &Vec3::from_f64(0.5, 0.5, 1.0));

        assert_eq!(a.hit(&r, 0.01, f64::MAX), true);
    }

    #[test]
    fn test_no_hit() {
        let a = AABB::new(Vec3::from_f64(0.0, 0.0, 0.0), Vec3::from_f64(1.0, 1.0, 1.0));
        let r = Ray::new(&Vec3::from_f64(0.5, 5.0, -1.0), &Vec3::from_f64(0.0, 0.0, 1.0));

        assert_eq!(a.hit(&r, 0.01, f64::MAX), false);
    }
}
