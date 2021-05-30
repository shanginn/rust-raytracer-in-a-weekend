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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin().clone() - self.center.clone();
        let a = Vec3::dot(&ray.direction(), &ray.direction());
        let b = Vec3::dot(&oc, &ray.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant: f64 = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;

            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center.clone()) / self.radius;
                rec.material = self.material.clone();

                return true;
            }

            let temp = (-b + discriminant.sqrt()) / a;

            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center.clone()) / self.radius;
                rec.material = self.material.clone();

                return true;
            }
        }

        false
    }
}

pub struct HittableList<T>
{
    pub list: Vec<T>
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for o in &self.list {
            if o.hit(ray, t_min, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }
}