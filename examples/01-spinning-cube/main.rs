#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate cgmath;
extern crate gl;
extern crate noire;
extern crate notify;

use gl::types::*;

use cgmath::{Matrix4, SquareMatrix};

use noire::mesh;
use noire::mesh::mesh::Mesh;
use noire::mesh::cube::Cube;
use noire::render::shader::*;
use noire::render::program::*;
use noire::render::traits::*;
use noire::render::vertex::*;
use noire::render::vertex_buffer::*;
use noire::render::index_buffer::*;
use noire::render::window::{OpenGLWindow,RenderWindow,Window};

use notify::*;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let mut window = RenderWindow::create(600, 600, "Hello This is window")
        .expect("Failed to create Render Window");

    // create shader program
    let vertex_file = String::from("./examples/01-spinning-cube/shaders/vertex.glsl");
    let fragment_file = String::from("./examples/01-spinning-cube/shaders/fragment.glsl");
    let program: Program = Program::compile_from_files(&vertex_file, &fragment_file).unwrap();

    // create vertex data
    let mesh = Mesh::create(Cube::create(1.0));
    let vao = mesh.vao;

    let start_time = Instant::now();

    let model_view_proj = Matrix4::<f32>::identity();

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
        program.uniform("u_modelViewProjection", model_view_proj.into());

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
