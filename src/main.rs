mod vec3;
mod ray;
mod camera;
mod objects;
mod material;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rand::prelude::*;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::camera::Camera;
use crate::objects::*;
use crate::material::*;

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p = Vec3::unit(1.0);

    while p.squared_length() >= 1.0 {
        p = 2.0 * Vec3(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        ) - Vec3::unit(1.0)
    }

    p
}

fn color<M: Material>(ray: &Ray, world: &HittableList<Sphere<M>>, depth: u32) -> Vec3 {
    let mut record = HitRecord {
        t: 0.0,
        p: Vec3::unit(0.0),
        normal: Vec3::unit(0.0),
        material: M::create(Vec3::unit(0.0))
    };

    if world.hit(ray, 0.001, f64::MAX, &mut record) {
        let mut scattered = Ray {
            a: Vec3::unit(0.0),
            b: Vec3::unit(0.0)
        };

        let mut attenuation = Vec3::unit(0.0);

        let scatters = record.material.scatter(
            ray,
            &record,
            &mut attenuation,
            &mut scattered
        );

        //println!("{:?}", attenuation);

        if depth < 50 && scatters {
            attenuation * color(&scattered, world, depth + 1)
        } else {
            Vec3::unit(0.0)
        }
    } else {
        let unit_direction = Vec3::unit_vector(ray.direction().clone());
        let t = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - t) * Vec3::unit(1.0) + t * Vec3(0.5, 0.7, 1.0)
    }
}

fn main() {
    let path = Path::new("img.ppm");
    let file = File::create(&path).expect("Err create file");
    let mut rng = rand::thread_rng();

    let xn = 200;
    let yn = 100;
    let sn = 100;

    write!(&file, "P3\n{} {}\n255\n", xn, yn).expect("Err writing header");

    let max_color = 255.99;

    let world = HittableList {
        list: vec![
            Sphere {
                center: Vec3(0.0, 0.0, -1.0),
                radius: 0.5,
                material: Lambertian {
                    albedo: Vec3(0.8, 0.3, 0.3)
                }
            },
            Sphere {
                center: Vec3(0.0, -100.5, -1.0),
                radius: 100.0,
                material: Lambertian {
                    albedo: Vec3(0.8, 0.8, 0.0)
                }
            },
            // Box::new(Sphere {
            //     center: Vec3(1.0, 0.0, -1.0),
            //     radius: 0.5,
            //     material: Metal {
            //         albedo: Vec3(0.8, 0.6, 0.2)
            //     }
            // }),
            // Box::new(Sphere {
            //     center: Vec3(-1.0, 0.0, -1.0),
            //     radius: 100.0,
            //     material: Metal {
            //         albedo: Vec3(0.8, 0.8, 0.8)
            //     }
            // }),
        ]
    };

    let camera = Camera::default();

    for j in (0..yn).rev() {
        for i in 0..xn {
            let mut col = Vec3::unit(0.0);
            for _s in 0..sn {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / xn as f64;
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / yn as f64;
                let r = camera.get_ray(u, v);

                let _p = r.point_at_parameter(2.0);

                col += color(&r, &world, 0);
            }

            col = (col / sn as f64).sqrt();

            writeln!(&file, "{}", col * max_color).expect("Error writing line");
        }
    }
}
