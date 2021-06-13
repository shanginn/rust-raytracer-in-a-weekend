mod vec3;
mod ray;
mod camera;
mod objects;
mod material;
mod scene;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::vec3::Vec3;
use crate::camera::Camera;
use crate::scene::*;

const WIDTH: usize = 1000;
const HEIGHT: usize = 600;
const RAYS: usize = 300;
const THREADS: usize = 12;

fn main() {
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

    let world = get_random_scene();

    let img = render(
        WIDTH,
        HEIGHT,
        THREADS,
        RAYS,
        world,
        camera
    );

    let path = Path::new("img.ppm");

    write_file(path, img);
}

fn write_file(path: &Path, img: Vec<Vec3>) {
    let file = File::create(&path).expect("Err create file");

    write!(&file, "P3\n{} {}\n255\n", WIDTH, HEIGHT).expect("Err writing header");

    for pixel in img.iter() {
        writeln!(&file, "{}", *pixel * 255.99).expect("Error writing line");
    }
}