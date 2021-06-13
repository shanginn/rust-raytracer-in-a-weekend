use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::*;

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = *ray.origin() - self.center;

        let a = Vec3::dot(&ray.direction(), &ray.direction());
        let b = Vec3::dot(&oc, &ray.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius.powi(2);

        let discriminant = b.powi(2) - a * c;
        let discriminant_sqrt = discriminant.sqrt();

        if discriminant > 0.0 {
            let temp = (-b - discriminant_sqrt) / a;

            if t_max > temp && temp > t_min {
                let point = ray.point_at_parameter(temp);

                return Some(HitRecord {
                    t: temp,
                    p: point,
                    normal: (point - self.center) / self.radius,
                    material: self.material.clone(),
                });
            }

            let temp = (-b + discriminant_sqrt) / a;

            if t_max > temp && temp > t_min {
                let point = ray.point_at_parameter(temp);

                return Some(HitRecord {
                    t: temp,
                    p: point,
                    normal: (point - self.center) / self.radius,
                    material: self.material.clone(),
                });
            }
        }

        return None;
    }
}

pub struct HittableList<T>
{
    pub list: Vec<T>
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for o in &self.list {
            if let Some(hit) = o.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;

                hit_anything = Some(hit);
            }
        }

        hit_anything
    }
}