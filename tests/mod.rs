use ray_tracing_one_weekend::{Vec3,vec3};

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

    let cross_vec = vec3::cross(&v1, &v2);

    assert_eq!(cross_vec[0], v1.y() * v2.z() - v1.z() * v2.y());
    assert_eq!(cross_vec[1], v1.z() * v2.x() - v1.x() * v2.z());
    assert_eq!(cross_vec[2], v1.x() * v2.y() - v1.y() * v2.x());

    let versor = vec3::unit_vector(&v1);

    let norm = f32::sqrt(2.0 * 2.0 + 1.0 * 1.0 + 0.0 * 0.0);
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

    assert_eq!(vec.lenght_squared(), 14.0_f32);
    assert_eq!(vec.lenght(), 14.0_f32.sqrt());

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
