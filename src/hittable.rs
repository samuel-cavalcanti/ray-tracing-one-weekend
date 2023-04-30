use std::{ops::Range, rc::Rc};

use crate::{ Float, Point3, Ray, Vec3, material::Material};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: Float,
    pub front_face: bool,
    pub material:Rc<dyn Material>
}
pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Range<Float>) -> Option<HitRecord>;
}

