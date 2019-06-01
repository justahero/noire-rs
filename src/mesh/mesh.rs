use super::cube::Cube;

use render::Primitive;
use render::vertex::VertexArrayObject;
use render::vertex_buffer::VertexBuffer;
use render::index_buffer::IndexBuffer;

pub struct Mesh {
    pub vao: VertexArrayObject,
    // TODO add a few more properties, local object matrix
}

impl Mesh {
    pub fn create(cube: Cube) -> Mesh {
        let mut vao = VertexArrayObject::new();

        vao.add_vb(VertexBuffer::create(&cube.vertices, cube.num_components() as u32, Primitive::Triangles));
        // vao.add_vb(VertexBuffer::create(&cube.texcoords, cube.num_vertices() as u32, Primitive::Triangles));
        vao.add_ib(IndexBuffer::create(&cube.indices));

        Mesh { vao }
    }
}
