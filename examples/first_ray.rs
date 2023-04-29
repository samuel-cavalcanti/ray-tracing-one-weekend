use ray_tracing_one_weekend::{Color, write_color, Ray, vec3, Point3, Vec3};

pub fn ray_color(ray:&Ray)->Color{

    let unit_direction = vec3::unit_vector(&ray.direction);

    let t = 0.5*(unit_direction.y() +1.0);

    (1.0 -t)*Color::new(1.0,1.0,1.0) + t*Color::new(0.5,0.7,1.0)

}
fn main() {
    // Image
    let image_width = 400;
    let aspect_ratio = 16.0/9.0;
    let image_height = ( image_width as f32 / aspect_ratio) as u32;
    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio *viewport_height;
    let focal_lenght = 1.0;

    let origin = Point3::new(0.0,0.0,0.0);
    let horizontal = Vec3::new(viewport_width,0.0,0.0);
    let vertical = Vec3::new(0.0,viewport_height,0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0,0.0,focal_lenght);

    //render 
    let progress_bar = indicatif::ProgressBar::new(image_height.into());
    let last_index_height = image_height - 1;
    let last_index_width = image_width - 1;
    for j in (0..image_height).rev() {
        progress_bar.inc(1);

        for i in 0..image_width {
            let u = i as f32 / (last_index_height as f32);
            let v = j as f32 / (last_index_width as f32);

            let ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let color = ray_color(&ray);
            
            write_color(&mut imgbuf[(i, last_index_height - j)],color);
        }
    }
    progress_bar.finish_with_message("done");

    imgbuf.save("images/first_ray.png").unwrap();
}
