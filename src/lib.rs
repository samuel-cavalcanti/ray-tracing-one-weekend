mod ray;
pub mod vec3;
mod hittable;
mod sphere;

pub use ray::Ray;
pub use vec3::Vec3;
pub use hittable::{Hittable,HitRecord};
pub use sphere::Sphere;

pub type Float = f64;
pub type Color = Vec3;
pub type Point3 = Vec3;

pub fn write_color(p: &mut image::Rgb<u8>, c: Color) {
    let normalize = |v| (v * 255.999) as u8;
    let ir = normalize(c[0]);
    let ig = normalize(c[1]);
    let ib = normalize(c[2]);
    *p = image::Rgb([ir, ig, ib]);
}
