#![allow(unused_variables)]
pub mod camera;

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

// Vector4::new(c0r0, c0r1, c0r2, c0r3),
// Vector4::new(c1r0, c1r1, c1r2, c1r3),
// Vector4::new(c2r0, c2r1, c2r2, c2r3),
// Vector4::new(c3r0, c3r1, c3r2, c3r3))

//  0,  1,  2,  3,
//  4,  5,  6,  7,
//  8,  9, 10, 11,
// 12, 13, 14, 15,

// export function getRotation(out, mat) {
//   // Algorithm taken from http://www.euclideanspace.com/maths/geometry/rotations/conversions/matrixToQuaternion/index.htm
//   let trace = mat[0] + mat[5] + mat[10];
//   let S = 0;

//   if (trace > 0) {
//     S = Math.sqrt(trace + 1.0) * 2;
//     out[3] = 0.25 * S;
//     out[0] = (mat[6] - mat[9]) / S;
//     out[1] = (mat[8] - mat[2]) / S;
//     out[2] = (mat[1] - mat[4]) / S;
//   } else if ((mat[0] > mat[5])&(mat[0] > mat[10])) {
//
//     S = Math.sqrt(1.0 + mat[0] - mat[5] - mat[10]) * 2;
//     out[3] = (mat[6] - mat[9]) / S;
//     out[0] = 0.25 * S;
//     out[1] = (mat[1] + mat[4]) / S;
//     out[2] = (mat[8] + mat[2]) / S;
//   } else if (mat[5] > mat[10]) {
//     S = Math.sqrt(1.0 + mat[5] - mat[0] - mat[10]) * 2;
//     out[3] = (mat[8] - mat[2]) / S;
//     out[0] = (mat[1] + mat[4]) / S;
//     out[1] = 0.25 * S;
//     out[2] = (mat[6] + mat[9]) / S;
//   } else {
//     S = Math.sqrt(1.0 + mat[10] - mat[0] - mat[5]) * 2;
//     out[3] = (mat[1] - mat[4]) / S;
//     out[0] = (mat[8] + mat[2]) / S;
//     out[1] = (mat[6] + mat[9]) / S;
//     out[2] = 0.25 * S;
//   }

//   return out;
// }
