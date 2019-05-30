use super::cube::Cube;
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
        vao.add_vb(VertexBuffer::create(&cube.vertices, 2, gl::TRIANGLE_STRIP));
        vao.add_ib(IndexBuffer::create(&cube.indices));

        Mesh { vao }
    }
}
