use ray_tracing_one_weekend::{Color,Float, write_color};

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);
    
    let progress_bar = indicatif::ProgressBar::new(image_height.into());
    let last_index_height = image_height - 1;
    let last_index_width = image_width - 1;
    for j in (0..image_height).rev() {
        progress_bar.inc(1);
        for i in 0..image_width {
            let r = i as Float / (last_index_height as Float);
            let g = j as Float / (last_index_width as Float);

            let b = 0.25;
            let color = Color::new(r,g,b);
            
            write_color(&mut imgbuf[(i, last_index_height - j)],color);
        }
    }
    
    progress_bar.finish_with_message("done");
    imgbuf.save("images/first_image.png").unwrap();
}
