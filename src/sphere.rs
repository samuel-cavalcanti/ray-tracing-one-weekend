use std::ops::Range;

use crate::{vec3, Float, HitRecord, Hittable, Point3, Ray};

pub struct Sphere {
    center: Point3,
    radius: Float,
}

impl Sphere {
    pub fn new(center: Point3, radius: Float) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval:Range<Float>) -> Option<HitRecord> {
        // t²b⋅b + 2tb⋅(A−C)+(A−C)⋅(A−C)−r²=0 (quadratic equation)
        // Where b  is ray direction,
        // A is the ray origin,
        // C is the sphere center
        // r is the sphere  radius
        let oc = ray.origin - self.center;
        // solving the ax² + bx + c = 0 equation
        let a = ray.direction.lenght_squared();
        let half_b = vec3::dot(&oc, &ray.direction);
        let c = oc.lenght_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }


        let sqrtd = discriminant.sqrt();

        let root_a = (-half_b - sqrtd) / a;

        let root_b = (-half_b + sqrtd) / a;

        //let is_out_interval = |root| root <= t_min || t_max <= root;


        let make_record = |root| {
            let p = ray.at(root);
            let outward_normal = (p - self.center) / self.radius;
            HitRecord::form_sphere(p, root, outward_normal, &ray.direction)
        };
        
        match (interval.contains(&root_a),interval.contains(&root_b)){
            (false, false) => None,
            (false, true) => Some(make_record(root_b)) ,
            (true, _) => Some(make_record(root_a)),
        }
    }
}