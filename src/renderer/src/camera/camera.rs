use cgmath::{EuclideanSpace, Matrix4, One, Point3, Quaternion, SquareMatrix, Vector3};

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
        Self {
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
    /// Sets the perspective from given aspect ratio
    pub fn perspective(&mut self, aspect: f32) -> &mut Self {
        self.aspect = aspect;
        self
    }

    /// Sets field of view
    pub fn fov(&mut self, fov: f32) -> &mut Self {
        self.fov = fov;
        self
    }

    /// Sets the z near & far values
    pub fn depth(&mut self, znear: f32, zfar: f32) -> &mut Self {
        self.znear = znear;
        self.zfar = zfar;
        self
    }

    /// Sets the look at matrix from eye, center and up vector
    pub fn look_at(&mut self, eye: Point3<f32>, center: Point3<f32>, up: Vector3<f32>) -> &mut Self {
        self.position = eye;
        self.view = Matrix4::look_at(eye, center, up);
        self.orientation = convert_to_quaternion(&self.view);
        self.update_view();
        self
    }

    fn update_view(&mut self) -> &mut Self {
        let rotation = Matrix4::from(self.orientation);
        let translation = Matrix4::from_translation(self.position.to_vec());
        self.view = rotation * translation.invert().unwrap();
        self
    }
}
