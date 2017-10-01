use cgmath::Matrix4;
use cgmath::One;

#[derive(Debug)]
pub struct Camera {
    pub znear: f32,
    pub zfar: f32,
    pub aspect: f32,
    pub fov: f32,
    projection: Matrix4<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            znear: 0.1,
            zfar: 100.0,
            fov: 60.0,
            aspect: 1.0,
            projection: Matrix4::one(),
        }
    }

    pub fn perspective(&mut self, fov: f32, aspect: f32, znear: f32, zfar: f32) -> &mut Camera {
        self.fov = fov;
        self.aspect = aspect;
        self.znear = znear;
        self.zfar = zfar;
        self
    }
}
