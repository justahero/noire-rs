use cgmath::prelude::InnerSpace;
use cgmath::{Deg, Matrix4, PerspectiveFov, Point3, Rad, Transform, Vector3};

use math::Color;
use super::Perspective;

pub struct Spotlight {
    pub view: Matrix4<f32>,
    pub projection: Matrix4<f32>,
    pub pos: Point3<f32>,
    pub target: Point3<f32>,
    pub direction: Vector3<f32>,
    pub color: Color,
}

fn get_direction(eye: &Point3<f32>, target: &Point3<f32>) -> Vector3<f32> {
    (eye - target).normalize()
}

fn get_projection(perspective: Perspective) -> Matrix4<f32> {
    Matrix4::from(PerspectiveFov {
        fovy: Rad::from(Deg(perspective.fov)),
        aspect: perspective.aspect,
        near: perspective.near,
        far: perspective.far,
    })
}

impl Spotlight {
    pub fn new(color: Color) -> Self {
        let pos = Point3{ x: 0.0, y: 0.0, z: 0.0 };
        let target = Point3{ x: -1.0, y: 0.0, z: 0.0 };

        Spotlight {
            view: Matrix4::one(),
            projection: get_projection(Perspective::default()),
            pos,
            target,
            direction: get_direction(&pos, &target),
            color,
        }
    }

    pub fn set_perspective(&mut self, perspective: Perspective) -> &mut Self {
        self.projection = get_projection(perspective);
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