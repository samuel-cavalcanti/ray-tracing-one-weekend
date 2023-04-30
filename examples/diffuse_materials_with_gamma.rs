use ray_tracing_one_weekend::{
    random,
    vec3::{self, random_in_init_sphere},
     Camera, Color, Float, HitRecord, Hittable, Point3, Ray, Sphere, write_color_wih_gamma_correction,
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
    if depth  <=0 {
        let black = Color::new(0.0, 0.0, 0.0);
        return black;
    }


    if let Some(rec) = hit_anything(ray, world) {
        let target =  rec.normal + random_in_init_sphere();
        let new_ray = Ray::new(rec.p, target );
        let new_color = ray_color(&new_ray, world, depth - 1);
        return 0.5 * new_color;
    }

    let white = Color::new(1.0, 1.0, 1.0);
    let blue = Color::new(0.5, 0.7, 1.0);

    let unit_direction = vec3::unit_vector(&ray.direction);

    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * white + t * blue
}

fn main() {
    // Image
    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0;
    let image_height = (image_width as Float / aspect_ratio) as u32;
    let samples_per_pixel = 100.0;
    let depth = 50;
    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    // World

    let world = vec![
        Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0),
    ];

    // Camera

    let camera = Camera::default();

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

    imgbuf.save("images/diffuse_material_with_gamma.png").unwrap();
}
