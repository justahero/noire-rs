use cgmath::prelude::InnerSpace;
use cgmath::{Deg, Matrix4, One, PerspectiveFov, Point3, Rad, Vector3};

use crate::math::Color;

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

fn get_projection(fov: f32, aspect: f32, znear: f32, zfar: f32) -> Matrix4<f32> {
    Matrix4::from(PerspectiveFov {
        fovy: Rad::from(Deg(fov)),
        aspect,
        near: znear,
        far: zfar,
    })
}

impl Spotlight {
    pub fn new(color: Color) -> Self {
        let pos = Point3{ x: 0.0, y: 0.0, z: 0.0 };
        let target = Point3{ x: -1.0, y: 0.0, z: 0.0 };

        Spotlight {
            view: Matrix4::one(),
            projection: get_projection(60.0, 1.0, 0.1, 50.0),
            pos,
            target,
            direction: get_direction(&pos, &target),
            color,
        }
    }

    pub fn set_perspective(&mut self, fov: f32, aspect: f32, znear: f32, zfar: f32) -> &mut Self {
        self.projection = get_projection(fov, aspect, znear, zfar);
        self
    }

    pub fn set_lookat(&mut self, eye: Point3<f32>, target: Point3<f32>, up: Vector3<f32>) -> &mut Self {
        self.pos       = eye;
        self.target    = target;
        self.direction = get_direction(&self.pos, &self.target);
        self.view      = Matrix4::look_at_lh(eye, target, up);
        self
    }
}