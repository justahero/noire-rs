use super::{Cube, Plane};

use crate::math::Color;
use crate::render::{Primitive, RenderError};
use crate::render::{IndexBuffer, VertexBuffer, VertexArrayObject, VertexAttributeDescriptor, vertex_buffer::VertexType};

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

        let attributes = vec![
            VertexAttributeDescriptor::new("position", VertexType::Float, 3, 0),
        ];
        vao.add_vb(VertexBuffer::create(&cube.vertices, attributes));

        let attributes = vec![
            VertexAttributeDescriptor::new("normal", VertexType::Float, 3, 1),
        ];
        vao.add_vb(VertexBuffer::create(&cube.normals, attributes));
        vao.add_ib(IndexBuffer::create(&cube.indices)?);

        Ok(Mesh {
            vao,
            color,
        })
    }

    /// Creates a Mesh object from a Plane
    pub fn create_plane(plane: Plane, color: Color) -> Result<Mesh, RenderError> {
        let mut vao = VertexArrayObject::new(Primitive::Triangles);

        let attributes = vec![
            VertexAttributeDescriptor::new("position", VertexType::Float, 3, 0),
        ];
        vao.add_vb(VertexBuffer::create(&plane.vertices, attributes));

        let attributes = vec![
            VertexAttributeDescriptor::new("normal", VertexType::Float, 3, 1),
        ];
        vao.add_vb(VertexBuffer::create(&plane.normals, attributes));
        vao.add_ib(IndexBuffer::create(&plane.indices)?);

        Ok(Mesh {
            vao,
            color,
        })
    }
}
