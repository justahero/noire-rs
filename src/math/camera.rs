use cgmath::{Deg, EuclideanSpace, Euler, InnerSpace, Matrix4, PerspectiveFov, Point3, Rad,
             Rotation, SquareMatrix, Vector3, Quaternion};
use cgmath::vec3;
use cgmath::One;

use math::convert_to_quaternion;

#[derive(Debug)]
pub struct Camera {
    pub znear: f32,
    pub zfar: f32,
    pub aspect: f32,
    pub fov: f32,

    pub projection: Matrix4<f32>,
    pub view: Matrix4<f32>,
    pub position: Point3<f32>,
    pub orientation: Quaternion<f32>,
}

impl Default for Camera {
    fn default() -> Self {
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
}

impl Camera {
    /// Creates a new default Camera object
    pub fn new() -> Camera {
        Default::default()
    }

    /// Set perspective projection matrix
    pub fn perspective(&mut self, fov: f32, aspect: f32, znear: f32, zfar: f32) -> &mut Self {
        self.fov = fov;
        self.aspect = aspect;
        self.znear = znear;
        self.zfar = zfar;
        // self.projection = perspective(Rad::from(Deg(fov)), aspect, znear, zfar);
        self.projection = Matrix4::from(PerspectiveFov {
            fovy: Rad::from(Deg(fov)),
            aspect: aspect,
            near: znear,
            far: zfar,
        });
        self
    }

    // TODO change up vector to a default constant
    pub fn lookat(&mut self, eye: Point3<f32>, center: Point3<f32>, up: Vector3<f32>) -> &mut Self {
        self.position = eye;
        self.view = Matrix4::look_at(eye, center, up);
        self.orientation = convert_to_quaternion(&self.view);
        self.update_view()
    }

    pub fn invert_view(&self) -> Option<Matrix4<f32>> {
        self.view.invert()
    }

    pub fn yaw(&self) -> f32 {
        let euler = Euler::from(self.orientation);
        euler.y.0
    }

    pub fn pitch(&self) -> f32 {
        let euler = Euler::from(self.orientation);
        euler.x.0
    }

    pub fn roll(&self) -> f32 {
        let euler = Euler::from(self.orientation);
        euler.z.0
    }

    pub fn forward(&self) -> Vector3<f32> {
        let conjugate = self.orientation.conjugate().normalize();
        conjugate.rotate_vector(vec3(0.0, 0.0, -1.0))
    }

    pub fn right(&self) -> Vector3<f32> {
        let conjugate = self.orientation.conjugate().normalize();
        conjugate.rotate_vector(vec3(1.0, 0.0, 0.0))
    }

    pub fn set_position(&mut self, pos: Point3<f32>) -> &mut Self {
        self.position = pos;
        self.update_view()
    }

    pub fn set_orientation(&mut self, orientation: Quaternion<f32>) {
        self.orientation = orientation;
        self.update_view();
    }

    fn update_view(&mut self) -> &mut Self {
        let rotation = Matrix4::from(self.orientation);
        let translation = Matrix4::from_translation(self.position.to_vec());
        self.view = rotation * translation.invert().unwrap();
        self
    }
}
