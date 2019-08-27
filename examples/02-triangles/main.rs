#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate gl;
extern crate noire;
extern crate notify;

use gl::types::*;

use noire::render::{Bindable, Drawable, Primitive, Program, Shader, VertexArrayObject};
use noire::render::{IndexBuffer, VertexBuffer};
use noire::render::{OpenGLWindow, RenderWindow, Size, Window};

use std::time::{Duration, Instant};

static VERTICES: [GLfloat; 8] = [-1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0];
static INDICES: [GLuint; 6] = [0, 1, 2, 2, 3, 1];

fn main() {
    let window_size = Size::new(600, 600);
    let mut window = RenderWindow::create(&window_size, "Hello This is window")
        .expect("Failed to create Render Window");

    // create shader program
    let vertex_file = String::from("./examples/02-triangles/shaders/vertex.glsl");
    let fragment_file = String::from("./examples/02-triangles/shaders/fragment.glsl");
    let mut program: Program = Program::compile_from_files(&vertex_file, &fragment_file).unwrap();

    // create vertex data
    let vb = VertexBuffer::create(&VERTICES, 2, Primitive::TriangleStrip);
    let ib = IndexBuffer::create(&INDICES).unwrap();

    let mut vao = VertexArrayObject::new().unwrap();
    vao.add_vb(vb);
    vao.add_ib(ib);

    let start_time = Instant::now();

    loop {
        let now = Instant::now();
        let elapsed = now.duration_since(start_time);
        let elapsed = (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9) as f32;

        window.clear(0.3, 0.3, 0.3, 1.0);

        let size = window.get_framebuffer_size();

        program.bind();
        // is there a way to use deref coercion to not specify Uniform type?
        program.uniform("u_resolution", size.into());
        program.uniform("u_time", elapsed.into());

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
