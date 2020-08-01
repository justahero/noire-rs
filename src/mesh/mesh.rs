use super::{Cube, Plane};

use math::Color;
use render::{Primitive, RenderError};
use render::{IndexBuffer, VertexBuffer, VertexArrayObject};

/// A basic mesh structure that contains vertex data and some
/// properties to be used in a scene
pub struct Mesh {
    /// the vertex array object
    pub vao: VertexArrayObject,
    /// the ambient color of the Mesh
    pub color: Color,
}

impl Mesh {
    /// Creates a Mesh object from a Cube
    pub fn create_cube(cube: Cube, color: Color) -> Result<Mesh, RenderError> {
        let mut vao = VertexArrayObject::new(Primitive::Triangles);

        vao.add_vb(VertexBuffer::create(&cube.vertices, 3));
        vao.add_vb(VertexBuffer::create(&cube.normals, 3));
        vao.add_ib(IndexBuffer::create(&cube.indices)?);

        Ok(Mesh {
            vao,
            color,
        })
    }

    /// Creates a Mesh object from a Plane
    pub fn create_plane(plane: Plane, color: Color) -> Result<Mesh, RenderError> {
        let mut vao = VertexArrayObject::new(Primitive::Triangles);

        vao.add_vb(VertexBuffer::create(&plane.vertices, 3));
        vao.add_vb(VertexBuffer::create(&plane.normals, 3));
        vao.add_ib(IndexBuffer::create(&plane.indices)?);

        Ok(Mesh {
            vao,
            color,
        })
    }
}
