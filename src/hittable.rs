use crate::{Float, Point3, Ray, Vec3, vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: Float,
    pub front_face:bool
}
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}

impl HitRecord {

    pub fn form_sphere(p:Point3,t:Float,outward_normal:Vec3,ray_directio:&Vec3)->HitRecord{
        
            let front_face = vec3::dot(ray_directio,&outward_normal) < 0.0;

            let normal = match front_face{
                true => outward_normal,
                false => -outward_normal,
            };

            HitRecord {
                p,
                normal,
                t ,
                front_face
            }
    }
}
