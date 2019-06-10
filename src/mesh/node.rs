use cgmath::One;
use cgmath::{Matrix4, Vector3};

use super::{Mesh};
use render::{Bindable, Drawable};

/// A basic node, can be used in a scene
pub struct Node {
    /// the mesh to render
    pub mesh: Mesh,
    /// the local model view matrix
    pub model_view: Matrix4<f32>,
}

impl Node {
    /// Creates a new Node instance, wrapping a Mesh
    pub fn new(mesh: Mesh) -> Self {
        Node {
            mesh,
            model_view: Matrix4::one(),
        }
    }

    /// Translate this Node
    pub fn translate(&mut self, pos: Vector3<f32>) -> &mut Self {
        self.model_view = self.model_view * Matrix4::from_translation(pos);
        self
    }
}

impl Drawable for Node {
    fn draw(&self) {
        self.mesh.vao.bind();
        self.mesh.vao.draw();
        self.mesh.vao.unbind();
    }
}
