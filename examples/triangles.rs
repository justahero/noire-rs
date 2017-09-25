extern crate gl;
extern crate noire;

use gl::types::*;

use noire::render::shader::*;
use noire::render::program::*;
use noire::render::traits::*;
use noire::render::vertex::*;
use noire::render::window::RenderWindow;

use std::time::Instant;

static VERTICES: [GLfloat; 8] = [-1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0];

fn main() {
    let mut window = RenderWindow::create(600, 600, "Hello This is window")
        .expect("Failed to create Render Window");

    let vertex_shader =
        create_shdaer_from_file("./examples/shaders/vertex.glsl", gl::VERTEX_SHADER).unwrap();
    let fragment_shader =
        create_shdaer_from_file("./examples/shaders/fragment.glsl", gl::FRAGMENT_SHADER).unwrap();
    let program = Program::create(vertex_shader, fragment_shader).unwrap();

    println!("UNIFORMS: {:?}", program.uniforms);
    println!("ATTRIBUTES: {:?}", program.attributes);

    let vb = VertexBuffer::create(&VERTICES, 2, gl::TRIANGLE_STRIP);
    let mut vao = VertexArrayObject::new();
    vao.add_vb(vb);

    let start_time = Instant::now();

    while !window.should_close() {
        let now = Instant::now();
        let elapsed = now.duration_since(start_time);
        let elapsed = (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9) as f32;

        window.clear(0.3, 0.3, 0.3, 1.0);

        let (width, height) = window.get_size();

        // render square
        program.bind();
        program.uniform2f("u_resolution", width as f32, height as f32);
        // program.uniform1f("u_time", elapsed as f32);
        program.bind_frag_location("out_color", 0);

        vao.bind();
        vao.draw();
        vao.unbind();
        program.unbind();

        window.swap_buffers();

        window.poll_events();
    }
}
