use cgmath::{Matrix4, Point3, Vector3, Quaternion};
use cgmath::One;

use math::convert_to_quaternion;

#[derive(Debug)]
pub struct Camera {
    pub znear: f32,
    pub zfar: f32,
    pub aspect: f32,
    pub fov: f32,

    projection: Matrix4<f32>,
    view: Matrix4<f32>,
    position: Point3<f32>,
    orientation: Quaternion<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            znear: 0.1,
            zfar: 100.0,
            fov: 60.0,
            aspect: 1.0,
            projection: Matrix4::one(),
            view: Matrix4::one(),
            position: Point3::new(0.0, 0.0, 0.0),
            orientation: Quaternion::one(),
        }
    }

    pub fn perspective(&mut self, fov: f32, aspect: f32, znear: f32, zfar: f32) -> &mut Camera {
        self.fov = fov;
        self.aspect = aspect;
        self.znear = znear;
        self.zfar = zfar;
        self
    }

    pub fn lookat(
        &mut self,
        eye: Point3<f32>,
        center: Point3<f32>,
        up: Vector3<f32>,
    ) -> &mut Camera {
        self.position = eye.clone();
        self.view = Matrix4::look_at(eye, center, up);
        self.orientation = convert_to_quaternion(&self.view);
        self
    }
}
