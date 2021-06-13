use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::objects::*;
use crate::material::*;
use rand::Rng;
use rand::prelude::ThreadRng;
use crate::camera::Camera;

pub fn get_random_scene() -> HittableList<Sphere> {
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

fn color(ray: &Ray, world: &HittableList<Sphere>, depth: u32, rng: &mut ThreadRng) -> Vec3 {
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
        let unit_direction = Vec3::unit_vector(*ray.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);

        Vec3::unit(1.0 - t) + t * Vec3(0.5, 0.7, 1.0)
    }
}

pub fn render(
    width: usize,
    height: usize,
    threads: usize,
    rays: usize,
    world: HittableList<Sphere>,
    camera: Camera
) -> Vec<Vec3> {
    let mut img: Vec<Vec3> = vec![Vec3::unit(0.0); width * height];

    let chunk_size = width * height / threads;

    crossbeam::scope(|scope| {
        for (chunk_index, pixels_chunk) in img.chunks_mut(chunk_size).enumerate() {
            let w = &world;
            let c = &camera;

            scope.spawn(move |_| {
                let mut rng = rand::thread_rng();
                for (pixel_index, pixel) in pixels_chunk.iter_mut().enumerate() {
                    let position = pixel_index + (chunk_index * chunk_size);
                    let x = position % width;
                    let y = height - position / width;

                    let mut col = Vec3::unit(0.0);
                    for _ in 0..rays {
                        let u = (x as f64 + rng.gen_range(0.0..1.0)) / width as f64;
                        let v = (y as f64 + rng.gen_range(0.0..1.0)) / height as f64;
                        let r = c.get_ray(u, v, &mut rng);

                        col += color(&r, w, 0, &mut rng);
                    }

                    *pixel = (col / rays as f64).sqrt();
                }
            });
        }
    }).unwrap();

    img
}