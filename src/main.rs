mod vec3;
use vec3::*;
mod ray;
use ray::*;
mod hittable;
mod sphere;
mod hittable_list;
use hittable_list::*;
use hittable::*;

use sphere::*;
use std::rc::Rc;
use image::*;

fn ray_color(r: &Ray, world: &HittableList) -> Color3 {
    let mut rec = HitRecord::new();
    if world.hit(*r, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Vec3::from_f64(1.0, 1.0, 1.0));
    }

    let unit_dir = Vec3::unit(&r.direction());
    let t = 0.5*(unit_dir.y() + 1.0);
    return (1.0-t)*Color3::from_f64(1.0, 1.0, 1.0)+t*Color3::from_f64(0.5, 0.7, 1.0);
}

fn write_color(col: &Color3) -> Rgb<u8> {
    Rgb([(col.x()*255.99) as u8, (col.y()*255.99) as u8, (col.z()*255.99) as u8])
}

fn main() {
    const ASPECT_RATIO : f64 = 16.0/9.0;
    const IMAGE_WIDTH : u32 = 400;
    const IMAGE_HEIGHT : u32 = (400 as f64 / ASPECT_RATIO) as u32;

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Vec3::from_f64(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::from_f64(0.0, -100.5, -1.0), 100.0)));

    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let viewport_height = 2.0;
    let viewport_width = 2.0*ASPECT_RATIO;
    let focal_len = 1.0;

    let origin = Vec3::from_f64(0.0, 0.0, 0.0);
    let horizontal = Vec3::from_f64(viewport_width, 0.0, 0.0);
    let vertical = Vec3::from_f64(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::from_f64(0.0, 0.0, focal_len);

    for y in 0..IMAGE_HEIGHT {
        println!("Scanlines remaining: {}", IMAGE_HEIGHT-y);
        for x in 0..IMAGE_WIDTH {
            let u = x as f64 / (IMAGE_WIDTH-1) as f64;
            let v = y as f64 / (IMAGE_HEIGHT-1) as f64;

            let dir = lower_left_corner+u*horizontal+v*vertical - origin;
            let r = Ray::new(&origin, &dir);

            let col = ray_color(&r, &world);
            img.put_pixel(x, IMAGE_HEIGHT-1-y, write_color(&col));
        }
    }


    let _ = img.save("test.png");
    println!("Done.");
}
