use cgmath::Matrix4;
use cgmath::One;

use super::cube::Cube;
use super::plane::Plane;

use render::Primitive;
use render::vertex::VertexArrayObject;
use render::vertex_buffer::VertexBuffer;
use render::index_buffer::IndexBuffer;

pub struct Mesh {
    pub vao: VertexArrayObject,
    // TODO add a few more properties, local object matrix
    pub model_view: Matrix4<f32>,
}

impl Mesh {
    /// Creates a Mesh object from a Cube
    pub fn create_cube(cube: Cube) -> Mesh {
        let mut vao = VertexArrayObject::new();

        vao.add_vb(VertexBuffer::create(&cube.vertices, 3, Primitive::Triangles));
        vao.add_vb(VertexBuffer::create(&cube.normals, 3, Primitive::Triangles));
        vao.add_vb(VertexBuffer::create(&cube.texcoords, 2, Primitive::Triangles));
        vao.add_ib(IndexBuffer::create(&cube.indices));

        Mesh {
            vao,
            model_view: Matrix4::one()
        }
    }

    /// Creates a Mesh object from a Plane
    pub fn create_plane(plane: Plane) -> Mesh {
        let mut vao = VertexArrayObject::new();

        vao.add_vb(VertexBuffer::create(&plane.vertices, 3, Primitive::Triangles));
        vao.add_vb(VertexBuffer::create(&plane.normals, 3, Primitive::Triangles));
        vao.add_vb(VertexBuffer::create(&plane.texcoords, 2, Primitive::Triangles));
        vao.add_ib(IndexBuffer::create(&plane.indices));

        Mesh {
            vao,
            model_view: Matrix4::one()
        }
    }
}
