use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        fov: f64,
        aspect: f64
    ) -> Self {
        let theta = fov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = Vec3::unit_vector(look_from - look_at);
        let u = Vec3::unit_vector(view_up.cross(&w));
        let v = w.cross(&u);

        Camera {
            lower_left_corner: look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: look_from,
        }
    }

    // pub fn default() -> Self {
    //     Camera {
    //         origin: Vec3::unit(0.0),
    //         lower_left_corner: Vec3(-2.0, -1.0, -1.0),
    //         horizontal: Vec3(4.0, 0.0, 0.0),
    //         vertical: Vec3(0.0, 2.0, 0.0),
    //     }
    // }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            a: self.origin,
            b: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin
        }
    }
}