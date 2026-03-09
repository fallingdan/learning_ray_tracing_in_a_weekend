use crate::{
    hittable::{self, Hittable},
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &Ray,
        ray_tmin: f64,
        ray_tmax: f64,
        hit_record: &mut hittable::HitRecord,
    ) -> bool {
        let oc: Vec3 = self.center - ray.origin();
        let a: f64 = ray.direction().length_squared();
        let h = Vec3::dotproduct(&ray.direction(), &oc);
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            let root = (h + sqrtd) / a;
            if root <= ray_tmin || root <= ray_tmax {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = ray.at(hit_record.t);
        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);

        true
    }
}
