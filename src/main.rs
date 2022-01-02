mod vec3;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;
mod util;
mod ray;

use vec3::*;
use ray::*;

use hittable_list::*;
use hittable::*;
use camera::*;

use sphere::*;
use std::rc::Rc;
use image::*;

fn ray_color(r: &Ray, world: &HittableList, depth: u32) -> Color3 {
    if depth <= 0 {
        return Vec3::from_f64(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.hit(*r, 0.001, f64::INFINITY, &mut rec) {
        let target = rec.p + rec.normal + Vec3::random_unit_vector();

        return 0.5 * ray_color(&Ray::new(&rec.p, &(target-rec.p)), world, depth-1);
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

fn main() {
    const ASPECT_RATIO : f64 = 16.0/9.0;
    const IMAGE_WIDTH : u32 = 400;
    const IMAGE_HEIGHT : u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    let samples_per_pixel = 100;
    let max_ray_depth = 50;

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Vec3::from_f64(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::from_f64(0.0, -100.5, -1.0), 100.0)));

    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let cam = Camera::new();

    for y in 0..IMAGE_HEIGHT {
        println!("Scanlines remaining: {}", IMAGE_HEIGHT-y);
        for x in 0..IMAGE_WIDTH {
            let mut pixel_col = Vec3::new();

            for _ in 0..samples_per_pixel {
                let x = x as f64;
                let y = y as f64;

                let dx = util::random_double();//*2.0 - 1.0;
                let dy = util::random_double();//*2.0 - 1.0;

                let u = (x + dx) / (IMAGE_WIDTH-1) as f64;
                let v = (y + dy) / (IMAGE_HEIGHT-1) as f64;

                let r = cam.get_ray(u, v);
                let col = ray_color(&r, &world, max_ray_depth);
                pixel_col += col;
            }

            img.put_pixel(x, IMAGE_HEIGHT-1-y, write_color(&pixel_col, samples_per_pixel));
        }
    }

    let _ = img.save("test.png");
    println!("Done.");
}
