# Ray Tracing in One Weekend

## Output an Image

```bash
cargo r --release --example=first_image 
```

First image                     |  expected
:-------------------------:|:----------------------------------:
![first image ](images/first_image.png) |  ![Expected ](https://raytracing.github.io/images/img-1.01-first-ppm-image.png)

## Rays, a Simple Camera, and Background

blue to white

```bash
cargo r --release --example=first_ray
```

blue to white                  |  expected
:-------------------------:|:---------:
![blue to white  ](images/first_ray.png) |  ![Expected](https://raytracing.github.io/images/img-1.02-blue-to-white.png)

## Adding a Sphere

```bash
cargo r --release --example=red_sphere
```

Red Sphere                    |  expected
:-------------------------:|:---------:
![Red Sphere ](images/red_sphere.png) |  ![Expected Red sphere](https://raytracing.github.io/images/img-1.03-red-sphere.png)

## Surface Normals and Multiple Objects

```bash
cargo r --release --example=normals_on_a_sphere
```

Surface Normals                     |  expected
:----------------------------------------------------------:|:---------------------------------------------------------------------------------------------------:
![Surface Normals](images/normal_on_sphere.png) |  ![Expected Surface Normals](https://raytracing.github.io/images/img-1.04-normals-sphere.png)

```bash
cargo r --release --example=normals_on_a_sphere_with_groud
```

Surface Normals With Ground                     |  expected
:-------------------------:|:---------:
![Surface Normals With Ground ](images/normal_on_sphere_with_ground.png) |  ![Expected Surface Normals With Ground](https://raytracing.github.io/images/img-1.05-normals-sphere-ground.png)

## Antialiasing

```bash
cargo r --release --example=antialiasing
```

without antialiasing         |  with antialiasing
:-------------------------:|:---------:
![without antialiasing](images/normal_on_sphere_with_ground.png) |  ![antialiasing](images/antialiasing.png)

## Diffuse Materials

```bash
cargo r --release --example=diffuse_materials
```

diffuse sphere         |  diffuse sphere with gamma correction
:-------------------------:|:---------:
![diffuse sphere](images/diffuse_material.png) |  ![antialiasing](images/diffuse_material_with_gamma.png)

Rendering the lambertian sphere

```bash
cargo r --release --example=lambertian_sphere
```

Rendering the diffuse sphere with hemispherical scattering

```bash
cargo r --release --example=diffuse_sphere_with_hemispherical_scattering 
```

|Lambertian sphere| diffuse sphere with hemispherical scattering|
:-------------------------:|:-------:|
![ Lambertian sphere](images/lambertian_shpere.png) | ![diffuse spheres with hemispherical scattering](images/diffuse_sphere_with_hemispherical_scattering.png)

## Metal

Rendering shiny metal

```bash
cargo r --release --example=shiny_metal
```

Rendering  fuzzed metal

```bash
cargo r --release --example=fuzzed_metal
```

|Shiny metal| fuzzed metal|
:-------------------------:|:-------:|
![ Shiny metal](images/shiny_metal.png) | ![fizzed metal](images/fuzzed_metal.png)
