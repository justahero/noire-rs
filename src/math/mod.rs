#![allow(unused_variables)]
pub mod camera;
pub mod color;

use cgmath::Point3;

#[macro_export]
macro_rules! color {
    ($r:expr) => {
        Color::new($r, $r, $r, 1.0)
    };
    ($r:expr, $g:expr, $b:expr) => {
        Color::new($r, $g, $b, 1.0)
    };
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        Color::new($r, $g, $b, $a)
    };
}

pub fn point3(x: f32, y: f32, z: f32) -> Point3<f32> {
    Point3 { x, y, z }
}

use cgmath::{Matrix4, Quaternion};

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
