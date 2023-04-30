use crate::{
    vec3,
    Color, HitRecord, Ray, Float,
};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<MaterialRecord>;
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
    fn scatter(&self, _ray_in: &Ray, hit_rec: &HitRecord) -> Option<MaterialRecord> {
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
    fuzzy:Float,
}

impl Metal {
    pub fn new(color: Color) -> Metal {
        Metal { albedo: color, fuzzy:0.0 }
    }
    pub fn with_fuzzy(color:Color,fuzzy:Float)-> Metal{
        Metal{albedo:color,fuzzy} 
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<MaterialRecord> {
        let unit = vec3::unit_vector(&ray_in.direction);
        let reflected = vec3::reflect(&unit, &hit_rec.normal);

        let ray_direction = reflected + self.fuzzy*vec3::random_in_init_sphere();

        match vec3::dot(&ray_direction, &hit_rec.normal) > 0.0 {
            true => Some(MaterialRecord {
                attenuation: self.albedo,
                scattered: Ray::new(hit_rec.p, ray_direction),
            }),
            false => None,
        }
    }
}
