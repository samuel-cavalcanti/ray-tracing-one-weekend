use crate::{
    vec3,
    Color, HitRecord, Ray,
};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_rec: HitRecord) -> Option<MaterialRecord>;
}

pub struct MaterialRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Lambertian {
        Lambertian { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_rec: HitRecord) -> Option<MaterialRecord> {
        let scatter_direction = hit_rec.normal + vec3::random_unit_vector();

        let scatter_direction = match scatter_direction.near_zero(){
            true => hit_rec.normal,
            false =>scatter_direction,
        };

        let scattered = Ray::new(hit_rec.p, scatter_direction);

        Some(MaterialRecord {
            attenuation: self.albedo,
            scattered,
        })
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(color: Color) -> Metal {
        Metal { albedo: color }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_rec: HitRecord) -> Option<MaterialRecord> {
        let unit = vec3::unit_vector(&ray_in.direction);
        let reflected = vec3::reflect(&unit, &ray_in.direction);

        match vec3::dot(&reflected, &hit_rec.normal) > 0.0 {
            true => Some(MaterialRecord {
                attenuation: self.albedo,
                scattered: Ray::new(hit_rec.p, reflected),
            }),
            false => None,
        }
    }
}
