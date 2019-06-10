use cgmath::{Matrix4, Vector3};
use cgmath::One;

use super::{Cube, Plane};

use math::Color;
use render::{Primitive, RenderError};
use render::{IndexBuffer, VertexBuffer, VertexArrayObject};

/// A basic mesh structure that contains vertex data and some
/// properties to be used in a scene
pub struct Mesh {
    /// the vertex array object
    pub vao: VertexArrayObject,
    /// the local model view matrix
    pub model_view: Matrix4<f32>,
    /// the ambient color of the Mesh
    pub ambient_color: Color,
}

impl Mesh {
    /// Creates a Mesh object from a Cube
    pub fn create_cube(cube: Cube) -> Result<Mesh, RenderError> {
        let mut vao = VertexArrayObject::new()?;

        vao.add_vb(VertexBuffer::create(&cube.vertices, 3, Primitive::Triangles));
        vao.add_vb(VertexBuffer::create(&cube.normals, 3, Primitive::Triangles));
        vao.add_vb(VertexBuffer::create(&cube.texcoords, 2, Primitive::Triangles));
        vao.add_ib(IndexBuffer::create(&cube.indices)?);

        Ok(Mesh {
            vao,
            model_view: Matrix4::one(),
            ambient_color: Color::rgb(1.0, 1.0, 1.0),
        })
    }

    /// Creates a Mesh object from a Plane
    pub fn create_plane(plane: Plane) -> Result<Mesh, RenderError> {
        let mut vao = VertexArrayObject::new()?;

        vao.add_vb(VertexBuffer::create(&plane.vertices, 3, Primitive::Triangles));
        vao.add_vb(VertexBuffer::create(&plane.normals, 3, Primitive::Triangles));
        vao.add_vb(VertexBuffer::create(&plane.texcoords, 2, Primitive::Triangles));
        vao.add_ib(IndexBuffer::create(&plane.indices)?);

        Ok(Mesh {
            vao,
            model_view: Matrix4::one(),
            ambient_color: Color::rgb(1.0, 1.0, 1.0),
        })
    }

    /// Translate this meshh
    pub fn translate(&mut self, pos: Vector3<f32>) -> &mut Self {
        self.model_view = self.model_view * Matrix4::from_translation(pos);
        self
    }
}
