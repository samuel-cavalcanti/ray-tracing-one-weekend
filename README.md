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
