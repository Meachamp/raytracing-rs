mod vec3;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;
mod util;
mod ray;
mod material;
mod lambertian;
mod metal;
mod dielectric;
mod triangle;
mod model;

use vec3::*;
use ray::*;

use hittable_list::*;
use hittable::*;
use camera::*;

use sphere::*;
use std::sync::Arc;
use image::*;
use std::time::{Instant};
use rayon::prelude::*;
use std::sync::RwLock;
use std::thread::{sleep, spawn};
use std::time::Duration;
use std::sync::atomic::{Ordering, AtomicU64};

fn ray_color(r: &Ray, world: &HittableList, depth: u32) -> Color3 {
    if depth <= 0 {
        return Vec3::from_f64(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.hit(*r, 0.001, f64::INFINITY, &mut rec) {
        //let target = rec.p + rec.normal + Vec3::random_unit_vector();
        //let target = rec.p + Vec3::random_in_hemisphere(&rec.normal);

        let mut attenuation = Vec3::new();
        let mut scattered = Ray::new(&Vec3::new(), &Vec3::new());

        let mat = rec.material.clone();
        if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth-1);
        }

        return Vec3::from_f64(0.0, 0.0, 0.0);
    }

    let unit_dir = Vec3::unit(&r.direction());
    let t = 0.5*(unit_dir.y() + 1.0);
    return (1.0-t)*Color3::from_f64(1.0, 1.0, 1.0)+t*Color3::from_f64(0.5, 0.7, 1.0);
}

fn write_color(col: &Color3, samples_per_pixel: u32) -> Rgb<u8> {
    let scale = 1.0/(samples_per_pixel as f64);

    let r = f64::clamp((col.x() * scale).sqrt(), 0.0, 0.999);
    let g = f64::clamp((col.y() * scale).sqrt(), 0.0, 0.999);
    let b = f64::clamp((col.z() * scale).sqrt(), 0.0, 0.999);

    Rgb([(r*256.0) as u8,
        (g*256.0) as u8,
        (b*256.0) as u8]
    )
}

fn random_world() -> HittableList {
    let mut world = HittableList::new();

    let mat_ground = Arc::new(lambertian::Lambertian::new(Vec3::from_f64(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(Vec3::from_f64(0.0, -1000.0, 0.0), 1000.0, mat_ground.clone())));

    for i in -11..11 {
        for j in -11..11 {
            let mat_type = util::random_double();

            let center = Vec3::from_f64((i as f64) + 0.9*util::random_double(),
                                        0.2,
                                        (j as f64) + 0.9*util::random_double());

            if (center - Vec3::from_f64(4.0, 0.2, 0.0)).length() > 0.9 && (center - Vec3::from_f64(-4.07, 0.0, 2.8)).length() > 1.5
                && (center - Vec3::from_f64(-4.0, 1.0, 0.0)).length() > 1.2
            {
                if mat_type < 0.8 {
                    let albedo = Vec3::random(0.0, 1.0) * Vec3::random(0.0, 1.0);
                    let mat = Arc::new(lambertian::Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));

                } else if mat_type < 0.95 {
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = util::random_range(0.0, 0.77);
                    let mat = Arc::new(metal::Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                } else {
                    let mat = Arc::new(dielectric::Dieletric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                }
            }
        }
    }

    let mat1 = Arc::new(dielectric::Dieletric::new(1.5));
    let mat2 = Arc::new(lambertian::Lambertian::new(Vec3::from_f64(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(metal::Metal::new(Vec3::from_f64(0.7, 0.6, 0.5), 0.0));

    world.add(Arc::new(Sphere::new(Vec3::from_f64(0.0, 1.0, 0.0), 1.0, mat1)));
    world.add(Arc::new(Sphere::new(Vec3::from_f64(-4.0, 1.0, 0.0), 1.0, mat3)));
    world.add(Arc::new(Sphere::new(Vec3::from_f64(4.0, 1.0, 0.0), 1.0, mat2)));

    world
}

static PROGRESS: AtomicU64 = AtomicU64::new(0);

fn main() {
    const ASPECT_RATIO : f64 = 16.0/9.0;
    const IMAGE_WIDTH : u32 = 400;
    const IMAGE_HEIGHT : u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    let samples_per_pixel = 50;
    let max_ray_depth = 5;

    let metal_mat = Arc::new(metal::Metal::new(Vec3::from_f64(59.0/255.0,102.0/255.0,57.0/255.0), 0.0));

    let mut world = random_world();
    //let mut world = HittableList::new();

    /*let v0 = Vec3::from_f64(-0.5, 0.0, 1.0);
    let v1 = Vec3::from_f64(-0.5, 1.0, 1.0);
    let v2 = Vec3::from_f64(-3.0, 1.0, 1.0);
    let mat_tri = Arc::new(lambertian::Lambertian::new(Vec3::from_f64(1.0, 0.0, 0.0)));
    world.add(Arc::new(triangle::Triangle::new(v0, v1, v2, mat_tri.clone())));*/

    let m = model::Model::new("cube2.obj", metal_mat.clone());
    world.add(Arc::new(m));

    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let lookfrom = Vec3::from_f64(-13.0, 3.0, 3.0);
    let lookat = Vec3::from_f64(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let cam = Camera::new(lookfrom,
                            lookat,
                            Vec3::from_f64(0.0, 1.0, 0.0),
                            ASPECT_RATIO,
                            20.0,
                            0.1,
                            dist_to_focus);

    let start = Instant::now();
    let mut cells = vec![Vec3::new(); (IMAGE_HEIGHT * IMAGE_WIDTH) as usize];

    spawn(move || {
        let start = Instant::now();
        loop {
            let pix = PROGRESS.load(Ordering::Relaxed);
            let pix = pix as f64 / (IMAGE_HEIGHT * IMAGE_WIDTH) as f64;
            let pix = pix * 100.0;
            let dur = Instant::now() - start;
            println!("Current progress: {:.2}%, {} seconds elapsed", dur.as_secs(), pix);
            let _ = sleep(Duration::from_secs(1));
        }
    });

    cells.par_iter_mut().enumerate().for_each(|(i, col_out)| {
        let mut pixel_col = Vec3::new();
        let x = (i as u32) % IMAGE_WIDTH;
        let y = (i as u32) / IMAGE_WIDTH;

        let x = x as f64;
        let y = y as f64;

        for _ in 0..samples_per_pixel {

            let dx = util::random_double();//*2.0 - 1.0;
            let dy = util::random_double();//*2.0 - 1.0;

            let u = (x + dx) / (IMAGE_WIDTH-1) as f64;
            let v = (y + dy) / (IMAGE_HEIGHT-1) as f64;

            let r = cam.get_ray(u, v);
            let col = ray_color(&r, &world, max_ray_depth);
            pixel_col += col;
        }

        *col_out = pixel_col;
        PROGRESS.fetch_add(1, Ordering::Relaxed);
    });

    println!("");
    println!("Writing image...");

    for i in 0..(IMAGE_HEIGHT*IMAGE_WIDTH) {
        let x = (i as u32) % IMAGE_WIDTH;
        let y = (i as u32) / IMAGE_WIDTH;

        img.put_pixel(x, IMAGE_HEIGHT-1-y, write_color(&cells[i as usize], samples_per_pixel));
    }

    let _ = img.save("test.png");
    println!("Done.");

    let end = Instant::now();
    println!("Render took {} secs", (end-start).as_secs());
}
