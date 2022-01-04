use tobj;
use crate::triangle::Triangle;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use std::sync::Arc;
use crate::hittable::*;

pub struct Model {
    tris: Vec<Triangle>
}

impl Model {
    pub fn new(path: &str, mat: Arc<dyn Material>) -> Self {
        let model = tobj::load_obj(
            path,
            &tobj::LoadOptions {
                ..Default::default()
            },
        );

        let (models, _) = model.expect("Model load failed.");


        let mut tris = Vec::new();

        for (_, model) in models.iter().enumerate() {
            let mesh = &model.mesh;

            let mut vertices = Vec::new();
            for chunk in mesh.positions.chunks(3) {
                vertices.push(Vec3::from_f64((chunk[0]) as f64, (chunk[1]) as f64, (chunk[2]) as f64));
            }

            if mesh.positions.len() % 3 != 0 {
                panic!("Mesh position vector not divible by component length!");
            }

            if mesh.indices.len() % 3 != 0 {
                panic!("Mesh is not triangulated!");
            }

            //println!("{:?}", vertices);
            for v_chunk in mesh.indices.chunks(3) {
                let v0 = vertices[v_chunk[0] as usize];
                let v1 = vertices[v_chunk[1] as usize];
                let v2 = vertices[v_chunk[2] as usize];
                println!("{} {} {}", v0, v1, v2);
                tris.push(Triangle::new(v0, v1, v2, mat.clone()));
            }
        }


        Self {
            tris
        }
    }
}

impl Hittable for Model {
    fn hit(&self, r: Ray, min: f64, max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp = HitRecord::new();
        let mut hit = false;
        let mut closest = max;

        for obj in &self.tris {
            if obj.hit(r, min, closest, &mut temp) {
                hit = true;
                closest = temp.t;
                *hit_record = temp.clone();
            }
        }

        hit
    }
}
