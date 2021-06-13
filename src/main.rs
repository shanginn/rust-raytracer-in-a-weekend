mod vec3;
mod ray;
mod camera;
mod objects;
mod material;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::camera::Camera;
use crate::objects::*;
use crate::material::*;
use rand::Rng;
use rand::prelude::ThreadRng;

fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
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

fn get_random_scene() -> HittableList<Sphere> {
    let mut rng = rand::thread_rng();
    let mut list = vec![
        Sphere {
            center: Vec3(0., -1000., 0.),
            radius: 1000.,
            material: Material::Lambertian(Lambertian {
                albedo: Vec3(0.5, 0.5, 0.5)
            })
        },
    ];

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0..1.0);
            let radius = rng.gen_range(0.1..0.3);
            let center = Vec3(
                a as f64 + 0.9 * rng.gen_range(0.0..1.0),
                radius,
                b as f64 + 0.9 * rng.gen_range(0.0..1.0)
            );

            let material;

            if (center - Vec3(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.5 {
                    material = Material::Lambertian(Lambertian {
                        albedo: Vec3(
                            rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0),
                            rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0),
                            rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0),
                        ),
                    });
                } else if choose_mat < 0.8 {
                    material = Material::Metal(Metal {
                        albedo: Vec3(
                            0.5 * rng.gen_range(1.0..4.0),
                            0.5 * rng.gen_range(1.0..4.0),
                            0.5 * rng.gen_range(1.0..4.0),
                        ),
                        fuzz: rng.gen_range(0.0..0.5)
                    });
                } else {
                    material = Material::Dielectric(Dielectric {
                        ref_idx: 1.5
                    });
                }

                list.push(Sphere { center, radius, material });
            }
        }
    }

    list.push(Sphere {
        center: Vec3(0., 1., 0.),
        radius: 1.,
        material: Material::Dielectric(Dielectric {
            ref_idx: 1.5
        })
    });

    list.push(Sphere {
        center: Vec3(4., 1., 0.),
        radius: 1.,
        material: Material::Lambertian(Lambertian {
            albedo: Vec3(0.4, 0.2, 0.1)
        })
    });

    list.push(Sphere {
        center: Vec3(-4., 1., 0.),
        radius: 1.,
        material: Material::Metal(Metal {
            albedo: Vec3(0.7, 0.6, 0.5),
            fuzz: 0.0
        })
    });

    HittableList { list }
}

pub fn color(ray: &Ray, world: &HittableList<Sphere>, depth: u32, rng: &mut ThreadRng) -> Vec3 {
    if let Some(record) = world.hit(ray, 0.001, f64::MAX) {
        let (attenuation, scattered, scatters) = record.material.scatter(
            ray,
            &record,
            rng
        );

        if depth < 50 && scatters {
            attenuation * color(&scattered, world, depth + 1, rng)
        } else {
            Vec3::unit(0.0)
        }
    } else {
        let unit_direction = Vec3::unit_vector(ray.direction().clone());
        let t = 0.5 * (unit_direction.y() + 1.0);

        Vec3::unit(1.0 - t) + t * Vec3(0.5, 0.7, 1.0)
    }
}

const WIDTH: usize = 800;
const HEIGHT: usize = 300;
const RAYS: usize = 100;
const THREADS: usize = 10;

fn main() {
    let path = Path::new("img.ppm");
    let file = File::create(&path).expect("Err create file");

    write!(&file, "P3\n{} {}\n255\n", WIDTH, HEIGHT).expect("Err writing header");

    let max_color = 255.99;

    let world = get_random_scene();

    let look_from = Vec3(-14.0, 2.0, -4.0);
    let look_at = Vec3(-4., 1., 0.);

    let camera = Camera::new(
        look_from,
        look_at,
        Vec3(0.0, 1.0, 0.0),
        15.0,
        WIDTH as f64 / HEIGHT as f64,
        0.15,
        (look_from - look_at).length()
    );

    let mut buffers: [Vec<Vec3>; THREADS] = [vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]];

    crossbeam::scope(|scope| {
        for (buf_index, buf) in buffers.iter_mut().enumerate() {
            let w = &world;
            let c = &camera;

            scope.spawn(move |_| {
                let mut rng = rand::thread_rng();
                let chunk = HEIGHT / THREADS;

                for j in (0..chunk).rev() {
                    for i in 0..WIDTH {
                        // println!("{} {} {}", buf_index, j, (j + (buf_index * chunk)));
                        let mut col = Vec3::unit(0.0);
                        for _ in 0..RAYS {
                            let u = (i as f64 + rng.gen_range(0.0..1.0)) / WIDTH as f64;
                            let v = ((j + (buf_index * chunk)) as f64 + rng.gen_range(0.0..1.0)) / HEIGHT as f64;
                            let r = c.get_ray(u, v, &mut rng);

                            col += color(&r, w, 0, &mut rng);
                        }

                        col = (col / RAYS as f64).sqrt();

                        buf.push(col);
                    }
                }
            });
        }
    }).unwrap();

    for b in buffers.iter().rev() {
        for v in b {
            writeln!(&file, "{}", *v * max_color).expect("Error writing line");
        }
    }
}
