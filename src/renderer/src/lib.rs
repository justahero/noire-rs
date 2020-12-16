extern crate bitflags;
extern crate shaderc;

mod buffer;
mod camera;
mod converter;
mod mesh;
mod pipeline;
mod render;
mod renderer;
mod shader;
mod surface;
mod texture;
mod vertex;
mod window;

pub use buffer::*;
pub use camera::*;
use cgmath::Point3;
pub use converter::*;
pub use mesh::*;
pub use pipeline::*;
pub use render::*;
pub use renderer::*;
pub use shader::*;
pub use surface::*;
pub use texture::*;
pub use vertex::*;
pub use window::*;

pub fn point3(x: f32, y: f32, z: f32) -> Point3<f32> {
    Point3 { x, y, z }
}
