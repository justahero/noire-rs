#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate gl;
extern crate noire;
extern crate notify;

use gl::types::*;

use noire::core::Timer;
use noire::render::{Bindable, Drawable, Primitive, Program, Shader, VertexArrayObject};
use noire::render::{IndexBuffer, VertexBuffer};
use noire::render::{OpenGLWindow, RenderWindow, Size, Window};

use std::time::{Duration, Instant};

fn main() {
    let window_size = Size::new(600, 600);
    let mut window = RenderWindow::create(&window_size, "Hello This is window")
        .expect("Failed to create Render Window");

    // create shader program
    let vertex_file = String::from("./examples/02-triangles/shaders/vertex.glsl");
    let fragment_file = String::from("./examples/02-triangles/shaders/fragment.glsl");
    let mut program: Program = Program::compile_from_files(&vertex_file, &fragment_file).unwrap();

    // create vertex data
    let mut vao = VertexArrayObject::screen_rect();

    let timer = Timer::now();

    loop {
        let elapsed = timer.elapsed_in_seconds() as f32;

        println!("ELAPSED: {}", elapsed);

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
