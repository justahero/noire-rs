use cgmath::prelude::InnerSpace;
use cgmath::Transform;
use cgmath::{Matrix4, Point3, Vector3};

use super::Perspective;

pub struct Spotlight {
    pub view: Matrix4<f32>,
    pub perspective: Perspective,
    pub pos: Point3<f32>,
    pub target: Point3<f32>,
    pub direction: Vector3<f32>,
}

fn get_direction(eye: &Point3<f32>, target: &Point3<f32>) -> Vector3<f32> {
    (eye - target).normalize()
}

impl Spotlight {
    pub fn new() -> Self {
        let pos = Point3{ x: 0.0, y: 0.0, z: 0.0 };
        let target = Point3{ x: -1.0, y: 0.0, z: 0.0 };

        Spotlight {
            view: Matrix4::one(),
            perspective: Perspective::default(),
            pos,
            target,
            direction: get_direction(&pos, &target),
        }
    }

    pub fn set_perspective(&mut self, perspective: Perspective) -> &mut Self {
        self.perspective = perspective;
        self
    }

    pub fn set_lookat(&mut self, eye: Point3<f32>, target: Point3<f32>, up: Vector3<f32>) -> &mut Self {
        self.pos       = eye;
        self.target    = target;
        self.direction = get_direction(&self.pos, &self.target);
        self.view      = Matrix4::look_at(eye, target, up);
        self
    }
}