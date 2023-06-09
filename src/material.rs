use crate::{vec3, Color, Float, HitRecord, Ray, Vec3, random};

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

        let scatter_direction = match scatter_direction.near_zero() {
            true => hit_rec.normal,
            false => scatter_direction,
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
    fuzzy: Float,
}

impl Metal {
    pub fn new(color: Color) -> Metal {
        Metal {
            albedo: color,
            fuzzy: 0.0,
        }
    }
    pub fn with_fuzzy(color: Color, fuzzy: Float) -> Metal {
        Metal {
            albedo: color,
            fuzzy,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<MaterialRecord> {
        let unit = vec3::unit_vector(&ray_in.direction);
        let reflected = vec3::reflect(&unit, &hit_rec.normal);

        let ray_direction = match self.fuzzy == 0.0 {
            true => reflected,
            false => reflected + self.fuzzy * vec3::random_in_init_sphere(),
        };

        match vec3::dot(&ray_direction, &hit_rec.normal) > 0.0 {
            true => Some(MaterialRecord {
                attenuation: self.albedo,
                scattered: Ray::new(hit_rec.p, ray_direction),
            }),
            false => None,
        }
    }
}

pub struct Dielectric {
    index_of_refraction: Float,
}

impl Dielectric {
    pub fn new(index_of_refraction: Float) -> Dielectric {
        Dielectric {
            index_of_refraction,
        }
    }

    fn schlicks_approx(cos_theta:Float,refraction:Float)->Float{
        
        let r_0 = (1.0 - refraction)/(1.0+refraction);

        let r_0 =  r_0*r_0;

        r_0 + (1.0-r_0)*Float::powi(1.0-cos_theta,5)
    }
}


impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<MaterialRecord> {
        let white = Color::new(1.0, 1.0, 1.0);

        let refraction_radio = match hit_rec.front_face {
            true => 1.0 / self.index_of_refraction,
            false => self.index_of_refraction,
        };

        let unit_direction = vec3::unit_vector(&ray_in.direction);
        let negative_unit = -unit_direction;
        let cost_theta = Float::min(vec3::dot(&negative_unit, &hit_rec.normal), 1.0);
        let sin_theta = Float::sqrt(1.0 - cost_theta * cost_theta);

        let can_refract = refraction_radio * sin_theta <= 1.0;
        let reflectance = Self::schlicks_approx(cost_theta,refraction_radio);

        let refract = || {
            let r_out_perp = refraction_radio * (unit_direction + cost_theta * hit_rec.normal);
            let r_out_parallel =
                -Float::sqrt(Float::abs(1.0 - r_out_perp.lenght_squared())) * hit_rec.normal;

            r_out_perp + r_out_parallel
        };

        let direction = match can_refract && reflectance <= random() {
            true => refract(),
            false => vec3::reflect(&unit_direction,&hit_rec.normal),
        };

        let scattered = Ray::new(hit_rec.p, direction);

        Some(MaterialRecord {
            attenuation: white,
            scattered,
        })
    }
}
