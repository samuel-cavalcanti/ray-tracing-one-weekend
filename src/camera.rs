use crate::{vec3, Float, Point3, Ray, Vec3};

#[derive(Copy,Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
    len_radius: Float,
}

impl Camera {
    pub fn get_ray(&self, s: Float, t: Float) -> Ray {
        let rd = self.len_radius * vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        let ray_direction =
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin;

        Ray::new(self.origin + offset, ray_direction - offset)
    }

    pub fn field_of_view(
        look_from: Point3,
        look_at: Point3,
        view_up: Vec3,
        vfov_in_degress: Float,
        aspect_ratio: Float,
        aperture: Float,
        focus_dist: Float,
    ) -> Camera {
        let theta = vfov_in_degress.to_radians();
        let h = Float::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = vec3::unit_vector(&(look_from - look_at));
        let u = vec3::unit_vector(&(vec3::cross(&view_up, &w)));
        let v = vec3::cross(&w, &u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            _w: w,
            len_radius: aperture / 2.0,
        }
    }
}
