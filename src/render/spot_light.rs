use cgmath::One;
use cgmath::{Matrix4, Vector3};

use super::Perspective;

pub struct Spotlight {
    pub projection: Matrix4<f32>,
    pub view: Matrix4<f32>,
    pub perspective: Perspective,
}

impl Spotlight {
    pub fn new() -> Self {
        Spotlight {
            projection: Matrix4::one(),
            view: Matrix4::one(),
            perspective: Perspective::default(),
        }
    }

    pub fn translate(&mut self, pos: Vector3<f32>) -> &mut Self {
        self.view = self.view * Matrix4::from_translation(pos);
        self
    }

    pub fn set_perspective(&mut self, perspective: Perspective) -> &mut Self {
        self.perspective = perspective;
        self
    }
}