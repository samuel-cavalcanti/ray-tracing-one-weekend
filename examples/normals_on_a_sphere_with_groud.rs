use ray_tracing_one_weekend::{
    vec3, write_color, Color, Float, HitRecord, Hittable, Point3, Ray, Sphere, Vec3,
};

fn hit_anything<H: Hittable>(ray: &Ray, world: &Vec<H>) -> Option<HitRecord> {
    let mut last_hit = None;
    let mut closed_so_far = Float::INFINITY;

    for hitable in world {
        let hit = hitable.hit(ray, 0.0..closed_so_far);
        if let Some(record) = hit {
            closed_so_far = record.t;
            last_hit = Some(record);
        }
    }

    last_hit
}
fn ray_color<H: Hittable>(ray: &Ray, world: &Vec<H>) -> Color {
    let white = Color::new(1.0, 1.0, 1.0);

    if let Some(rec) = hit_anything(ray, world) {
        return 0.5 * (rec.normal + white);
    }

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
    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    // World

    let world = vec![
        Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0),
    ];

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_lenght = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_lenght);

    //render
    let progress_bar = indicatif::ProgressBar::new(image_height.into());
    let last_index_height = image_height - 1;
    let last_index_width = image_width - 1;
    for j in (0..image_height).rev() {
        progress_bar.inc(1);

        for i in 0..image_width {
            let u = i as Float / (last_index_width as Float);
            let v = j as Float / (last_index_height as Float);

            let ray_direction = lower_left_corner + u * horizontal + v * vertical - origin;

            let ray = Ray::new(origin, ray_direction);
            let color = ray_color(&ray, &world);

            write_color(&mut imgbuf[(i, last_index_height - j)], color);
        }
    }
    progress_bar.finish_with_message("done");

    imgbuf
        .save("images/normal_on_sphere_with_ground.png")
        .unwrap();
}
