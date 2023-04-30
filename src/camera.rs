use crate::{Float, Point3, Ray, Vec3, vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn get_ray(&self, u: Float, v: Float) -> Ray {
        let ray_direction =
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;

        Ray::new(self.origin, ray_direction)
    }

    pub fn field_of_view(
        look_from: Point3,
        look_at: Point3,
        view_up:Vec3,
        vfov_in_degress: Float,
        aspect_ratio: Float,
    ) -> Camera {
        let theta = vfov_in_degress.to_radians();
        let h = Float::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        
         
        let w =  vec3::unit_vector(&(look_from - look_at));
        let u  = vec3::unit_vector(&(vec3::cross(&view_up,&w)));
        let v = vec3::cross(&w,&u);

        let origin = look_from;
        let horizontal = viewport_width*u;
        let vertical = viewport_height*v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_lenght = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_lenght);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}
