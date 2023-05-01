use std::rc::Rc;

use ray_tracing_one_weekend::{
    material::{Dielectric, Lambertian, Material, Metal},
    random, random_in_interval, vec3, write_color_wih_gamma_correction, Camera, Color, Float,
    HitRecord, Hittable, Point3, Ray, Sphere, Vec3,
};

fn hit_anything<H: Hittable>(ray: &Ray, world: &Vec<H>) -> Option<HitRecord> {
    let mut last_hit = None;
    let mut closed_so_far = Float::INFINITY;

    for hitable in world {
        let hit = hitable.hit(ray, 0.001..closed_so_far);
        if let Some(record) = hit {
            closed_so_far = record.t;
            last_hit = Some(record);
        }
    }

    last_hit
}
fn ray_color<H: Hittable>(ray: &Ray, world: &Vec<H>, depth: i32) -> Color {
    if depth <= 0 {
        let black = Color::new(0.0, 0.0, 0.0);
        return black;
    }

    if let Some(rec) = hit_anything(ray, world) {
        if let Some(material_rc) = rec.material.scatter(ray, &rec) {
            return material_rc.attenuation * ray_color(&material_rc.scattered, world, depth - 1);
        } else {
            let black = Color::new(0.0, 0.0, 0.0);
            return black;
        }
    }

    let white = Color::new(1.0, 1.0, 1.0);
    let blue = Color::new(0.5, 0.7, 1.0);

    let unit_direction = vec3::unit_vector(&ray.direction);

    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * white + t * blue
}

fn main() {
    // Image
    let image_width = 1200;
    let aspect_ratio = 3.0 / 2.0;
    let image_height = (image_width as Float / aspect_ratio) as u32;
    let samples_per_pixel = 500.0;
    let depth = 50;
    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    // World

    let world = random_scene();

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_focos = 10.0;
    let aperture = 0.1;

    let camera = Camera::field_of_view(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_focos,
    );

    //render
    let progress_bar = indicatif::ProgressBar::new(image_height.into());
    let last_index_height = image_height - 1;
    let last_index_width = image_width - 1;
    for j in (0..image_height).rev() {
        progress_bar.inc(1);

        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel as usize {
                let u = (i as Float + random()) / (last_index_width as Float);
                let v = (j as Float + random()) / (last_index_height as Float);

                let ray = camera.get_ray(u, v);
                let color = ray_color(&ray, &world, depth);
                pixel_color += color;
            }

            write_color_wih_gamma_correction(
                &mut imgbuf[(i, last_index_height - j)],
                pixel_color,
                samples_per_pixel,
            );
        }
    }
    progress_bar.finish_with_message("done");

    imgbuf.save("images/final_render.png").unwrap();
}

pub fn random_scene() -> Vec<Sphere> {
    let difuse_material = || {
        let albeto = Color::random() * Color::random();
        Rc::new(Lambertian::new(albeto))
    };

    let metal_material = || {
        let albeto = Color::random();
        let fuzzy = random_in_interval(0.0, 0.5);

        Rc::new(Metal::with_fuzzy(albeto, fuzzy))
    };

    let glass_material = Rc::new(Dielectric::new(1.5));

    let random_choose_material = || -> Rc<dyn Material> {
        let random_choose = random();
        if random_choose < 0.8 {
            difuse_material()
        } else if random_choose < 0.95 {
            metal_material()
        } else {
            glass_material.clone()
        }
    };

    let range_a = -11..11;

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    let brown = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));

    let metal = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5)));

    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material);

    let radius = 1.0;
    let big_glass_bal = Sphere::new(Point3::new(0.0, 1.0, 0.0), radius, glass_material.clone());

    let big_brown_spehre = Sphere::new(Point3::new(-4.0, 1.0, 0.0), radius, brown);

    let big_metal_sphere = Sphere::new(Point3::new(4.0, 1.0, 0.0), radius, metal);

    let mut world = Vec::with_capacity(range_a.len() * range_a.len() + 1 + 3);

    world.push(ground_sphere);
    world.push(big_glass_bal);
    world.push(big_brown_spehre);
    world.push(big_metal_sphere);

    for a in range_a {
        for b in -11..11 {
            let a = a as Float;
            let b = b as Float;

            let random_x = 0.9 * random();
            let random_z = 0.9 * random();

            let center = Point3::new(a + random_x, 0.2, b + random_z);
            let p = Point3::new(4.0, 0.2, 0.0);

            if (center - p).lenght() <= 0.9 {
                continue;
            }

            let material = random_choose_material();
            let sphere = Sphere::new(center, 0.2, material);

            world.push(sphere);
        }
    }

    world
}
