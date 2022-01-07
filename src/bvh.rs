use crate::*;
use ray::*;
use aabb::*;
use std::sync::Arc;
use hittable::*;
use hittable_list::*;
use std::cmp::Ordering;

fn box_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>, axis: usize) -> Ordering {
    let box_a = a.bounding_box().expect("A has no bounding box");
    let box_b = b.bounding_box().expect("B has no bounding box");

    box_a.min()[axis].partial_cmp(&box_b.min()[axis]).unwrap()
}

pub struct BVH {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bb: AABB
}

impl BVH {
    pub fn new(mut h: HittableList) -> Self {
        let l = h.list.len();
        BVH::new_internal(&mut h, 0, l)
    }

    fn new_internal(h: &mut HittableList, start: usize, end: usize) -> Self {
        let span = end - start;
        let axis = util::random_int(0, 2);

        let left;
        let right;

        match span {
            1 => {
                left = h.list[start].clone();
                right = h.list[start].clone();

            },
            2 => {
                if box_compare(h.list[start].clone(), h.list[start+1].clone(), axis as usize) == Ordering::Less {
                    left = h.list[start].clone();
                    right = h.list[start+1].clone();
                } else {
                    right = h.list[start].clone();
                    left = h.list[start+1].clone();
                }

            },
            _ => {
                h.list[start..end].sort_by(|a, b| box_compare(a.clone(), b.clone(), axis as usize));

                let mid = start + span/2;
                left = Arc::new(BVH::new_internal(h, start, mid));
                right = Arc::new(BVH::new_internal(h, mid, end));
            }
        };

        let box_l = left.bounding_box().expect("Left has no bounding box");
        let box_r = right.bounding_box().expect("Right has no bounding box");

        let bb = AABB::union(Some(box_l), Some(box_r)).unwrap();

        Self {
            left,
            right,
            bb
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: Ray, min: f64, max: f64, hit_record: &mut HitRecord) -> bool {
        if !self.bb.hit(&ray, min, max) {
            return false;
        }

        let hit_left = self.left.hit(ray, min, max, hit_record);

        let r_max = if hit_left {hit_record.t} else { max };
        let hit_right = self.right.hit(ray, min, r_max, hit_record);

        return hit_left || hit_right;
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bb.clone())
    }
}

unsafe impl Send for BVH {}
unsafe impl Sync for BVH {}
