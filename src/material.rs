use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::objects::*;
use crate::random_in_unit_sphere;

pub(crate) enum Materials {
    Lambertian(Lambertian),
    Metal(Metal),
}

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord<Self>,
        attenuation: &mut Vec3,
        scattered: &mut Ray
    ) -> bool where Self: Sized;

    fn create(albedo: Vec3) -> Self;

    fn get_albedo(&self) -> Vec3;
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v.clone() - 2.0 * Vec3::dot(v,n) * n.clone()
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord<Lambertian>, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = record.p + record.normal + random_in_unit_sphere();

        *scattered = Ray {
            a: record.p,
            b: target - record.p
        };

        *attenuation = self.albedo;

        true
    }

    fn create(albedo: Vec3) -> Self {
        Self { albedo }
    }

    fn get_albedo(&self) -> Vec3 {
        self.albedo
    }
}

pub struct Metal {
    pub albedo: Vec3,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord<Metal>, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = reflect(
            &Vec3::unit_vector(ray_in.direction().clone()),
            &record.normal
        );

        *scattered = Ray {
            a: record.p,
            b: reflected
        };


        *attenuation = self.albedo;

        Vec3::dot(scattered.direction(), &record.normal) > 0.0
    }

    fn create(albedo: Vec3) -> Self {
        Self { albedo }
    }

    fn get_albedo(&self) -> Vec3 {
        self.albedo
    }
}