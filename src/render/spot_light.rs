use cgmath::One;
use cgmath::{Matrix4, Vector3};

pub struct Spotlight {
    pub projection: Matrix4<f32>,
    pub view: Matrix4<f32>,
}

impl Spotlight {
    pub fn new() -> Self {
        Spotlight {
            projection: Matrix4::one(),
            view: Matrix4::one(),
        }
    }

    pub fn translate(&mut self, pos: Vector3<f32>) -> &mut Self {
        self.view = self.view * Matrix4::from_translation(pos);
        self
    }
}