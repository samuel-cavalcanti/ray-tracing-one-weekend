
fn main() {
    let image_width = 256;
    let image_height = 256;

    print!("P3\n {image_width} {image_height} \n255\n");
    

    let normalize = |v| (v * 255.999) as i32;

    for j in (0..image_height).rev(){
    
        for i in 0..image_width{

            let r =  i as f32 / (image_width as f32  -1.0);
            let g =  j as f32 / (image_height as f32  -1.0);
            let b = 0.25;
            
            let ir =  normalize(r);
            let ig =  normalize(g);
            let ib =  normalize(b);

            print!("{ir} {ig} {ib}\n");
        }
    }

   
    

}
