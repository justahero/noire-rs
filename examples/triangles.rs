extern crate gl;
extern crate noire;
extern crate notify;

use gl::types::*;

use noire::render::shader::*;
use noire::render::program::*;
use noire::render::traits::*;
use noire::render::vertex::*;
use noire::render::window::RenderWindow;

use notify::*;
use std::sync::mpsc::channel;
use std::time::Instant;

static VERTICES: [GLfloat; 8] = [-1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0];

fn compile_program(vertex_file: &str, fragment_file: &str) -> Program {
    let vertex_shader = create_shdaer_from_file(vertex_file, gl::VERTEX_SHADER).unwrap();
    let fragment_shader = create_shdaer_from_file(fragment_file, gl::FRAGMENT_SHADER).unwrap();
    Program::create(vertex_shader, fragment_shader).unwrap()
}

fn main() {
    let mut window = RenderWindow::create(600, 600, "Hello This is window")
        .expect("Failed to create Render Window");

    let vertex_file = "./examples/shaders/vertex.glsl";
    let fragment_file = "./examples/shaders/fragment.glsl";
    let program = compile_program(vertex_file, fragment_file);

    let vb = VertexBuffer::create(&VERTICES, 2, gl::TRIANGLE_STRIP);
    let mut vao = VertexArrayObject::new();
    vao.add_vb(vb);

    let start_time = Instant::now();

    loop {
        let now = Instant::now();
        let elapsed = now.duration_since(start_time);
        let elapsed = (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9) as f32;

        window.clear(0.3, 0.3, 0.3, 1.0);

        let (width, height) = window.get_size();

        program.bind();
        program.uniform2f("u_resolution", width as f32, height as f32);
        program.uniform1f("u_time", elapsed as f32);

        vao.bind();
        vao.draw();
        vao.unbind();

        program.unbind();

        window.swap_buffers();

        window.poll_events();
        if window.should_close() {
            return;
        }
    }
}
