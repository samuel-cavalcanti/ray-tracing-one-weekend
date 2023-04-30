mod camera;
mod hittable;
mod ray;
mod sphere;
pub mod material;
pub mod vec3;

pub use camera::Camera;
pub use hittable::{HitRecord, Hittable};
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::Vec3;

pub type Float = f64;
pub type Color = Vec3;
pub type Point3 = Vec3;

use rand::distributions::Distribution;

pub fn write_color(p: &mut image::Rgb<u8>, c: Color) {
    let normalize = |v| (v * 255.999) as u8;
    let ir = normalize(c[0]);
    let ig = normalize(c[1]);
    let ib = normalize(c[2]);
    *p = image::Rgb([ir, ig, ib]);
}

pub fn write_color_wih_samples(p: &mut image::Rgb<u8>, c: Color, samples_per_pixel: Float) {
    let mut color = c;
    color /= samples_per_pixel;

    let normalize = |v| (v * 256.0) as u8;
    let clamp = |v| {
        if v < 0.0 {
            return 0.0;
        }

        if v > 0.999 {
            return 0.999;
        }

        v
    };

    let ir = normalize(clamp(color[0]));
    let ig = normalize(clamp(color[1]));
    let ib = normalize(clamp(color[2]));

    *p = image::Rgb([ir, ig, ib]);
}

pub fn write_color_wih_gamma_correction(
    p: &mut image::Rgb<u8>,
    c: Color,
    samples_per_pixel: Float,
) {
    let mut color = c;
    color /= samples_per_pixel;

    color.slice[0] = color.slice[0].sqrt();
    color.slice[1] = color.slice[1].sqrt();
    color.slice[2] = color.slice[2].sqrt();

    let normalize = |v| (v * 256.0) as u8;
    let clamp = |v| {
        if v < 0.0 {
            return 0.0;
        }

        if v > 0.999 {
            return 0.999;
        }

        v
    };

    let ir = normalize(clamp(color[0]));
    let ig = normalize(clamp(color[1]));
    let ib = normalize(clamp(color[2]));

    *p = image::Rgb([ir, ig, ib]);
}

/// return random value between [0.0, 1.0]
pub fn random() -> Float {
    let mut rng = rand::thread_rng();
    let between = rand::distributions::Uniform::from(0.0..1.0);

    between.sample(&mut rng)
}

/// return random value between [min, max]
pub fn random_in_interval(min: Float, max: Float) -> Float {
    min + random() * (max - min)
}
