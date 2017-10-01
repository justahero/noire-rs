pub mod camera;

use cgmath::{Matrix4, Quaternion};
use cgmath::*;

pub fn convert_to_quaternion(_math: &Matrix4<f32>) -> Quaternion<f32> {
    Quaternion::one()
}
