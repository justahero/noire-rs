#![allow(unused_variables)]

pub mod camera;
pub mod color;
pub mod perlin_noise;
pub mod rect;
pub mod vector2;
pub mod vector3;

pub use self::camera::Camera;
pub use self::color::Color;
pub use self::perlin_noise::PerlinNoise;
pub use self::rect::Rect;
pub use self::vector2::Vector2;
pub use self::vector3::Vector3;

use cgmath::{Matrix, Matrix3, Matrix4, Point3, Quaternion, SquareMatrix};
use rand::{thread_rng, Rng};

/// Maps the given value to be between min..max range and maps to min_out..max_out
pub fn map(value: f32, min: f32, max: f32, out_min: f32, out_max: f32) -> f32 {
    out_min + (out_max - out_min) * (clamp(value, min, max) - min) / (max - min)
}

/// Clamps the given value between range min..max
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    min.max(value.min(max))
}

/// Generates a random value between 0.0 and the given max value
pub fn random_f32(max_value: f32) -> f32 {
    thread_rng().gen_range(0.0, max_value)
}

#[macro_export]
macro_rules! color {
    ($r:expr) => {
        Color::rgba($r, $r, $r, 1.0)
    };
    ($r:expr, $g:expr, $b:expr) => {
        Color::rgb($r, $g, $b)
    };
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        Color::rgba($r, $g, $b, $a)
    };
}

pub fn point3(x: f32, y: f32, z: f32) -> Point3<f32> {
    Point3 { x, y, z }
}

pub fn convert_to_quaternion(mat: &Matrix4<f32>) -> Quaternion<f32> {
    let trace = mat.x.x + mat.y.y + mat.z.z;

    let s;
    let x: f32;
    let y: f32;
    let z: f32;
    let w: f32;

    if trace > 0.0 {
        s = (trace + 1.0).sqrt() * 2.0;
        w = 0.25 * s;
        x = (mat.z.y - mat.y.z) / s;
        y = (mat.x.z - mat.z.x) / s;
        z = (mat.y.x - mat.x.y) / s;
    } else if (mat.x.x > mat.y.y) & (mat.x.x > mat.z.z) {
        s = (1.0 + mat.x.x - mat.y.y - mat.z.z).sqrt() * 2.0;
        w = (mat.z.y - mat.y.z) / s;
        x = 0.25 * s;
        y = (mat.x.y + mat.y.x) / s;
        z = (mat.x.z + mat.z.x) / s;
    } else if mat.y.y > mat.z.z {
        s = (1.0 + mat.y.y - mat.x.x - mat.z.z).sqrt() * 2.0;
        w = (mat.x.z - mat.z.x) / s;
        x = (mat.x.y + mat.y.x) / s;
        y = 0.25 * s;
        z = (mat.y.z + mat.z.y) / s;
    } else {
        s = (1.0 + mat.z.z - mat.x.x - mat.y.y) * 2.0;
        w = (mat.y.x - mat.x.y) / s;
        x = (mat.x.z + mat.z.x) / s;
        y = (mat.y.z + mat.z.y) / s;
        z = 0.25 * s;
    }

    Quaternion::new(w, x, y, z)
}

pub fn convert_to_matrix3(mat: &Matrix4<f32>) -> Matrix3<f32> {
    Matrix3::new(
        mat.x.x, mat.x.y, mat.x.z,
        mat.y.x, mat.y.y, mat.y.z,
        mat.z.x, mat.z.y, mat.z.z,
    )
}

/// Creates a normal matrix from a Matrix4, returning a Matrix3
pub fn normal_matrix(mat: &Matrix4<f32>) -> Matrix3<f32> {
    convert_to_matrix3(&mat.transpose().invert().unwrap())
}

#[cfg(test)]
mod tests {
    use super::{clamp, map};

    #[test]
    fn map_returns_result() {
        assert_eq!(map(0.0, 1.0, 4.0, 1.0, 4.0), 1.0);
        assert_eq!(map(2.0, 0.0, 4.0, 0.0, 4.0), 2.0);
        assert_eq!(map(5.0, 1.0, 4.0, 1.0, 4.0), 4.0);
    }

    #[test]
    fn clamp_returns() {
        assert_eq!(clamp(2.0, 0.0, 4.0), 2.0);
        assert_eq!(clamp(0.0, 1.0, 5.0), 1.0);
        assert_eq!(clamp(3.0, 1.0, 5.0), 3.0);
        assert_eq!(clamp(8.0, 1.0, 5.0), 5.0);
    }
}
