#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate gl;
extern crate noire;
extern crate notify;

use gl::types::*;

use noire::core::{FpsTimer, Timer};
use noire::render::{Bindable, Drawable, Primitive, Program, Shader, VertexArrayObject};
use noire::render::{IndexBuffer, VertexBuffer};
use noire::render::{OpenGLWindow, RenderWindow, Size, Window};

use std::time::{Duration, Instant};
use utils::app_dir;

const NUM_FRAMES: u32 = 360;

fn main() {
    let app_dir = app_dir().unwrap();
    let window_size = Size::new(640, 640);
    let mut window = RenderWindow::create(&window_size, "Metaballs").unwrap();

    // create shader program
    let vertex_file = app_dir.join("examples/09-metaballs/shaders/vertex.glsl");
    let fragment_file = app_dir.join("examples/09-metaballs/shaders/fragment.glsl");
    let mut program: Program = Program::compile_from_files(&vertex_file, &fragment_file).unwrap();

    // create vertex data
    let mut vao = VertexArrayObject::screen_rect();

    let timer = Timer::now();
    let mut fps_timer = FpsTimer::now();

    loop {
        let elapsed = timer.elapsed_in_seconds() as f32;

        fps_timer.next_frame();
        println!("Frame {} - FPS: {}", fps_timer.total_frames(), fps_timer.fps());

        window.reset_viewport();
        window.clear(0.3, 0.3, 0.3, 1.0);

        let size = window.get_framebuffer_size();

        program.bind();
        program.uniform("u_resolution", size.into());
        program.uniform("u_time", elapsed.into());

        // animate the points
        let t = (fps_timer.total_frames() as f32) / (NUM_FRAMES as f32);


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
