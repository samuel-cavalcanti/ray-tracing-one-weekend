use image::{DynamicImage, GenericImageView};
use ray_tracing_one_weekend::{vec3, Vec3, random, random_in_interval};

#[test]
fn test_utility_function_vec3() {
    let v1 = Vec3::new(2.0, 1.0, 0.0);
    let v2 = Vec3::new(1.0, 2.0, 3.0);

    let sum_v = v1 + v2;
    for v in sum_v.slice {
        assert_eq!(v, 3.0);
    }

    let min_v = sum_v - v2;

    for (v1_value, min_v_value) in v1.slice.iter().zip(min_v.slice) {
        assert_eq!(*v1_value, min_v_value);
    }

    let v3 = Vec3::new(3.0, 3.0, 3.0);

    let muti_v = v3 * 3.0;

    for v in muti_v.slice {
        assert_eq!(v, 9.0);
    }

    let muti_v = 3.0 * v3;

    for v in muti_v.slice {
        assert_eq!(v, 9.0);
    }

    let multi_v = v3 * (3.0, v1);

    for (v, v1_value) in multi_v.slice.iter().zip(v1.slice) {
        assert_eq!(*v, 3.0 * 3.0 * v1_value);
    }

    let multi_v = multi_v / 9.0;

    for (v, v1_value) in multi_v.slice.iter().zip(v1.slice) {
        assert_eq!(*v, v1_value);
    }

    let result = vec3::dot(&v1, &v2);

    assert_eq!(result, 2.0 * 1.0 + 1.0 * 2.0 + 0.0 * 3.0);

    let result_2 = vec3::dot(&v2, &v1);

    assert_eq!(result_2, result);

    let result = vec3::dot(&v1, &v1);
    assert_eq!(result, v1.lenght_squared());

    let cross_vec = vec3::cross(&v1, &v2);

    assert_eq!(cross_vec[0], v1.y() * v2.z() - v1.z() * v2.y());
    assert_eq!(cross_vec[1], v1.z() * v2.x() - v1.x() * v2.z());
    assert_eq!(cross_vec[2], v1.x() * v2.y() - v1.y() * v2.x());

    let versor = vec3::unit_vector(&v1);

    let norm = f64::sqrt(2.0 * 2.0 + 1.0 * 1.0 + 0.0 * 0.0);
    assert_eq!(versor[0], v1[0] / norm);
    assert_eq!(versor[1], v1[1] / norm);
    assert_eq!(versor[2], v1[2] / norm);
}
#[test]
fn test_vec3() {
    let vec = Vec3::new(1.0, 2.0, 3.0);

    assert_eq!(vec.x(), 1.0);
    assert_eq!(vec.y(), 2.0);
    assert_eq!(vec.z(), 3.0);

    let vec = -vec;

    assert_eq!(vec.x(), -1.0);
    assert_eq!(vec.y(), -2.0);
    assert_eq!(vec.z(), -3.0);

    assert_eq!(vec.lenght_squared(), 14.0);
    assert_eq!(vec.lenght(), 14.0_f64.sqrt());

    let mut vec = -vec;

    vec /= 3.0;

    assert_eq!(vec.x(), 1.0 / 3.0);
    assert_eq!(vec.y(), 2.0 / 3.0);
    assert_eq!(vec.z(), 1.0);

    vec *= 3.0;

    assert_eq!(vec.x(), 1.0);
    assert_eq!(vec.y(), 2.0);
    assert_eq!(vec.z(), 3.0);

    assert_eq!(vec[0], vec.x());
    assert_eq!(vec[1], vec.y());
    assert_eq!(vec[2], vec.z());
}

fn test_image(image: DynamicImage, expected_image: DynamicImage,image_name:&str) {
    let (width, height) = expected_image.dimensions();
    let (image_width, image_height) = expected_image.dimensions();

    assert_eq!(width, image_width);
    assert_eq!(height, image_height);

    for j in 0..height {
        for i in 0..width {
            let image_pixel = expected_image.get_pixel(i, j);

            let myimage_pixel = image.get_pixel(i, j);

            for i in 0..3{
                let expected_color_channel = image_pixel.0[i] as i32;
                let color_channel = myimage_pixel.0[i] as i32;
                let diff = expected_color_channel - color_channel;
                let acceptable_dif = 3;
                assert!( diff.abs() < acceptable_dif,"diff: {diff}, image: {image_name}");

            }
        }
    }
}

#[test]
fn test_images() -> Result<(), image::ImageError> {
    let images_path = vec![
        "images/first_image.png",
        "images/first_ray.png",
        "images/red_sphere.png",
        "images/normal_on_sphere.png",
        "images/normal_on_sphere_with_ground.png",
    ];

    let expected_images_path = vec![
        "images/img-1.01-first-ppm-image.png",
        "images/img-1.02-blue-to-white.png",
        "images/img-1.03-red-sphere.png",
        "images/img-1.04-normals-sphere.png",
        "images/img-1.05-normals-sphere-ground.png",
    ];

    assert_eq!(images_path.len(), expected_images_path.len());

    for (image_path, expected_path) in images_path.iter().zip(expected_images_path) {
        let image = image::open(image_path)?;
        let expected_image = image::open(expected_path)?;

        test_image(image, expected_image,image_path);
    }

    Ok(())
}

#[test]
// becase of the issue https://github.com/RayTracing/raytracing.github.io/issues/875
// this test should fail
fn test_diffuse_materials()-> Result<(), image::ImageError>{

    let images_path = vec![
        "images/diffuse_material.png",
        "images/lambertian_shpere.png",
        "images/diffuse_sphere_with_hemispherical_scattering.png",
        "images/shiny_metal.png",
        "images/fuzzed_metal.png",
    ];

    let expected_images_path = vec![
        "images/img-1.07-first-diffuse.png",
        "images/img-1.09-correct-lambertian.png",
        "images/img-1.10-rand-hemispherical.png",
        "images/img-1.11-metal-shiny.png",
        "images/img-1.12-metal-fuzz.png",
    ];

    assert_eq!(images_path.len(), expected_images_path.len());

    for (image_path, expected_path) in images_path.iter().zip(expected_images_path) {
        let image = image::open(image_path)?;
        let expected_image = image::open(expected_path)?;

        test_image(image, expected_image,image_path);
    }

    Ok(())
}

#[test]
fn test_random(){

    for _ in 0..1000{
        let value = random();
        assert!(value >= 0.0);
        assert!(value <= 1.0);

        let min = -1.0;
        let max = 1.0;
        let value = random_in_interval(-1.0,1.0);

        assert!(value >= min);
        assert!(value <= max);

    }

}
