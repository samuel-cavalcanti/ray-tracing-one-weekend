mod ray;
pub mod vec3;

pub use vec3::Vec3;
pub use ray::Ray;

pub type Color = Vec3;
pub type Point3 = Vec3;

pub fn write_color(p: &mut image::Rgb<u8>, c: Color) {
    let normalize = |v| (v * 255.999) as u8;
    let ir = normalize(c[0]);
    let ig = normalize(c[1]);
    let ib = normalize(c[2]);
    *p = image::Rgb([ir, ig, ib]);
}

