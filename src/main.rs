extern crate gl;
extern crate glfw;
extern crate regex;

pub mod noire;

use gl::types::*;

use noire::shader::*;
use noire::program::*;
use noire::traits::*;
use noire::vertex::*;
use noire::window::RenderWindow;

use std::time::Instant;

/* Shader sources */
static VERT_SHADER: &'static str = "#version 330\n\
    layout(location = 0) in vec2 position;\n\
    uniform float angle;\n\
    void main() {\n\
        gl_Position = vec4(position, 0.0, 1.0);\n\
    }\n";

static FRAG_SHADER: &'static str = "#version 330\n\
    out vec4 color;\n\
    void main() {\n\
        color = vec4(1, 0.15, 0.15, 0);\n\
    }\n";

static VERTICES: [GLfloat; 8] = [-1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0];

fn main() {
    let mut window = RenderWindow::create(600, 400, "Hello This is window")
        .expect("Failed to create Render Window");

    let vertex_shader = Shader::create(VERT_SHADER, gl::VERTEX_SHADER).unwrap();
    let pixel_shader = Shader::create(FRAG_SHADER, gl::FRAGMENT_SHADER).unwrap();
    let program = Program::create(vertex_shader, pixel_shader).unwrap();

    // initialize GL shader stuff
    let vb = VertexBuffer::create(&VERTICES, 2, gl::TRIANGLE_STRIP);
    let mut vao = VertexArrayObject::new();
    vao.add_vb(vb);

    let start_time = Instant::now();

    while !window.should_close() {
        let now = Instant::now();
        let elapsed = now.duration_since(start_time);
        let _elapsed = (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9) as f32;

        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // render square
        program.bind();
        vao.bind();
        vao.draw();
        vao.unbind();
        program.unbind();

        window.swap_buffers();

        window.poll_events();
    }
}
