use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::objects::*;
use crate::random_in_unit_sphere;
use rand::Rng;

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v.clone() - 2.0 * Vec3::dot(v,n) * n.clone()
}
fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64, refracted: &mut Vec3) -> bool {
    let uv = Vec3::unit_vector(*v);
    let dt = Vec3::dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        *refracted = ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt();

        true
    } else {
        false
    }
}

pub fn schlick (cos: f64, ref_idx: f64) -> f64 {
    let r0 = ((1. - ref_idx) / (1. + ref_idx)).powi(2);

    r0 + (1. - r0) * (1. - cos).powi(5)
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    fn scatter(&self, _ray_in: &Ray, record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = record.p + record.normal + random_in_unit_sphere();

        *scattered = Ray {
            a: record.p,
            b: target - record.p
        };

        *attenuation = self.albedo;

        true
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64
}

impl Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = reflect(
            &Vec3::unit_vector(ray_in.direction().clone()),
            &record.normal
        );

        let fuzz = if self.fuzz < 1.0 { self.fuzz } else { 1.0 };

        *scattered = Ray {
            a: record.p,
            b: reflected + fuzz * random_in_unit_sphere()
        };

        *attenuation = self.albedo;

        Vec3::dot(scattered.direction(), &record.normal) > 0.0
    }
}

#[derive(Clone)]
pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let outward_normal: Vec3;
        let reflected = reflect(ray_in.direction(), &record.normal);
        let ni_over_nt: f64;
        let mut refracted = Vec3::unit(0.0);
        let reflect_prod: f64;
        let cos: f64;

        *attenuation = Vec3::unit(1.0);

        if Vec3::dot(ray_in.direction(), &record.normal) > 0.0 {
            outward_normal = -record.normal;
            ni_over_nt = self.ref_idx;
            cos = self.ref_idx * Vec3::dot(ray_in.direction(), &record.normal) / ray_in.direction().length();
        } else {
            outward_normal = record.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cos = -Vec3::dot(ray_in.direction(), &record.normal) / ray_in.direction().length();
        }

        if refract(ray_in.direction(), &outward_normal, ni_over_nt, &mut refracted) {
            reflect_prod = schlick(cos, self.ref_idx);
        } else {
            reflect_prod = 1.0;
        }

        if rand::thread_rng().gen_range(0.0..1.0) < reflect_prod {
            *scattered = Ray { a: record.p, b: reflected };
        } else {
            *scattered = Ray { a: record.p, b: refracted };
        }

        true
    }
}

#[derive(Clone)]
pub enum Material {
    Metal(Metal),
    Lambertian(Lambertian),
    Dielectric(Dielectric)
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        match self {
            Self::Metal(m) => m.scatter(ray_in, record, attenuation, scattered),
            Self::Lambertian(m) => m.scatter(ray_in, record, attenuation, scattered),
            Self::Dielectric(m) => m.scatter(ray_in, record, attenuation, scattered),
        }
    }
}