use crate::vec3::Vec3;
use crate::ray::Ray;
use rand::Rng;

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p = Vec3::unit(1.0);

    while Vec3::dot(&p, &p) >= 1.0 {
        p = 2.0 * Vec3(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            0.0,
        ) - Vec3(1.0, 1.0, 0.0)
    }

    p
}

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        fov: f64,
        aspect: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let theta = fov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = Vec3::unit_vector(look_from - look_at);
        let u = Vec3::unit_vector(view_up.cross(&w));
        let v = w.cross(&u);

        let half_horizontal = half_width * focus_distance * u;
        let half_vertical = half_height * focus_distance * v;

        let lower_left_corner = look_from
            - half_horizontal
            - half_vertical
            - focus_distance * w;

        Camera {
            lower_left_corner,
            horizontal: 2.0 * half_horizontal,
            vertical: 2.0 * half_vertical,
            origin: look_from,
            lens_radius: aperture / 2.0,
            u, v,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        let a = self.origin + offset;

        Ray {
            a,
            b: self.lower_left_corner
                + s * self.horizontal
                + t * self.vertical
                - a
        }
    }
}