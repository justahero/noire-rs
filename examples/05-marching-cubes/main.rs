#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate cgmath;
extern crate gl;
extern crate noire;
extern crate notify;

use gl::types::*;

use noire::canvas::Canvas2D;
use noire::math::{Color, PerlinNoise, random_f32, Rect};
use noire::render::{OpenGLWindow, RenderWindow, Size, Window, Capability, Program, VertexArrayObject, Bindable, Drawable};
use std::time::Instant;
use cgmath::Vector2;

fn main() {
    let window_size = Size::new(800, 800);
    let mut window = RenderWindow::create(&window_size, "Hello This is window")
        .expect("Failed to create Render Window");

    // create shader program
    let vertex_file = String::from("./examples/05-marching-cubes/shaders/vertex.glsl");
    let fragment_file = String::from("./examples/05-marching-cubes/shaders/fragment.glsl");
    let mut program = Program::compile_from_files(&vertex_file, &fragment_file).unwrap();

    // create vertex data
    let mut vao = VertexArrayObject::screen_rect();

    let start_time = Instant::now();

    // randomly generate feature points
    let num_points = 10;
    let points: Vec<Vector2<f32>> = (0..num_points).into_iter().map( |_| {
        Vector2::new(random_f32(200.0), random_f32(200.0))
    }).collect();

    loop {
        let now = Instant::now();
        let elapsed = now.duration_since(start_time);
        let elapsed = (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9) as f32;

        let size = window.get_framebuffer_size();
        window.reset_viewport();
        window.clear(0.3, 0.3, 0.3, 1.0);

        program.bind();
        program.uniform("u_resolution", size.into());
        program.uniform("u_time", elapsed.into());
        program.uniform("u_featurePoints", points.clone().into());

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
