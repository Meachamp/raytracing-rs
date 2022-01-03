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

use vec3::*;
use ray::*;

use hittable_list::*;
use hittable::*;
use camera::*;

use sphere::*;
use std::rc::Rc;
use image::*;
use std::io::{Write, stdout};

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

        let mat = rec.material.clone().unwrap();
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

    let mat_ground = Rc::new(lambertian::Lambertian::new(Vec3::from_f64(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::from_f64(0.0, -100.5, -1.0), 100.0, mat_ground.clone())));

    for i in -11..11 {
        for j in -11..11 {
            let mat_type = util::random_double();

            let center = Vec3::from_f64((i as f64) + 0.9*util::random_double(),
                                        0.2,
                                        (j as f64) + 0.9*util::random_double());

            if (center - Vec3::from_f64(4.0, 0.2, 0.0)).length() > 0.9 {
                if mat_type < 0.8 {
                    let albedo = Vec3::random_unit_vector() * Vec3::random_unit_vector();
                    let mat = Rc::new(lambertian::Lambertian::new(albedo));
                    world.add(Rc::new(Sphere::new(center, 0.2, mat)));

                } else if mat_type < 0.95 {
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = util::random_range(0.0, 0.77);
                    let mat = Rc::new(metal::Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, mat)));
                } else {
                    let mat = Rc::new(dielectric::Dieletric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, mat)));
                }
            }
        }
    }

    let mat1 = Rc::new(dielectric::Dieletric::new(1.5));
    let mat2 = Rc::new(lambertian::Lambertian::new(Vec3::from_f64(0.4, 0.2, 0.1)));
    let mat3 = Rc::new(metal::Metal::new(Vec3::from_f64(0.7, 0.6, 0.5), 0.0));

    world.add(Rc::new(Sphere::new(Vec3::from_f64(0.0, 1.0, 0.0), 1.0, mat1)));
    world.add(Rc::new(Sphere::new(Vec3::from_f64(-4.0, 1.0, 0.0), 1.0, mat2)));
    world.add(Rc::new(Sphere::new(Vec3::from_f64(4.0, 1.0, 0.0), 1.0, mat3)));

    world
}

fn main() {
    const ASPECT_RATIO : f64 = 16.0/9.0;
    const IMAGE_WIDTH : u32 = 1920;
    const IMAGE_HEIGHT : u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    let samples_per_pixel = 100;
    let max_ray_depth = 50;

    /*let mat_ground = Rc::new(lambertian::Lambertian::new(Vec3::from_f64(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(lambertian::Lambertian::new(Vec3::from_f64(0.7, 0.3, 0.3)));
    let mat_left = Rc::new(dielectric::Dieletric::new(1.5));
    let mat_right = Rc::new(metal::Metal::new(Vec3::from_f64(0.8,0.6,0.2), 0.0));

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Vec3::from_f64(0.0, 0.0, -1.0), 0.5, mat_center)));
    world.add(Rc::new(Sphere::new(Vec3::from_f64(0.0, -100.5, -1.0), 100.0, mat_ground)));
    world.add(Rc::new(Sphere::new(Vec3::from_f64(-1.0, 0.0, -1.0), 0.5, mat_left.clone())));
    world.add(Rc::new(Sphere::new(Vec3::from_f64(-1.0, 0.0, -1.0), -0.4, mat_left)));
    world.add(Rc::new(Sphere::new(Vec3::from_f64(1.0, 0.0, -1.0), 0.5, mat_right)));*/

    let world = random_world();

    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let lookfrom = Vec3::from_f64(13.0, 2.0, 3.0);
    let lookat = Vec3::from_f64(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let cam = Camera::new(lookfrom,
                            lookat,
                            Vec3::from_f64(0.0, 1.0, 0.0),
                            ASPECT_RATIO,
                            20.0,
                            0.1,
                            dist_to_focus);

    for y in 0..IMAGE_HEIGHT {
        print!("Scanlines remaining: {: <8}\r", IMAGE_HEIGHT-y);
        let _ = stdout().flush();

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

    println!("");
    println!("Writing image...");
    let _ = img.save("test.png");
    println!("Done.");
}
