use ray_tracing_one_weekend::{Color, write_color};

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    let last_index_height = image_height - 1;
    let last_index_width = image_width - 1;
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let r = i as f32 / (last_index_height as f32);
            let g = j as f32 / (last_index_width as f32);
            let b = 0.25;
            let color = Color::new(r,g,b);
            
            write_color(&mut imgbuf[(i, last_index_height - j)],color);
        }
    }

    imgbuf.save("tests/first_image/image.png").unwrap();
}
