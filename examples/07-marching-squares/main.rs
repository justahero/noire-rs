#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate cgmath;
extern crate gl;
extern crate noire;
extern crate notify;

use gl::types::*;

use opensimplex::OpenSimplexNoise;
use noire::canvas::Canvas2D;
use noire::math::{Color, PerlinNoise, random_f32, Rect};
use noire::{core::{FpsTimer, Timer}, render::{OpenGLWindow, RenderWindow, Size, Window, Capability, Program, VertexArrayObject, Bindable, Drawable, Uniform}};
use std::time::Instant;
use cgmath::{Vector3, Vector2, Matrix3, InnerSpace, Rad, Matrix4, Vector4, Deg};

fn main() {
    let window_size = Size::new(600, 600);
    let rez = 10;

    let mut window = RenderWindow::create(&window_size, "Hello This is window")
        .expect("Failed to create Render Window");
    window.enable(Capability::ProgramPointSize);

    let timer = Timer::now();
    let mut fps_timer = FpsTimer::now();

    let mut canvas = Canvas2D::new(600, 600);
    let noise = PerlinNoise::new(0);

    let rez = 20;
    let cols = 1 + canvas.width / rez;
    let rows = 1 + canvas.height / rez;

    // initialize field values
    let mut field = vec![0.0; (cols * rows) as usize];
    for x in 0..cols {
        for y in 0..rows {
            let index = x + y * cols;
            field[index as usize] = random_f32(2.0).floor();
        }
    }

    loop {
        let elapsed = timer.elapsed_in_seconds() as f32;

        fps_timer.next_frame();
        println!("FPS: {}", fps_timer.fps());

        let size = window.get_framebuffer_size();
        window.reset_viewport();
        window.clear(0.4, 0.4, 0.4, 1.0);

        // render to canvas
        canvas.bind();
        canvas.set_pointsize(rez as f32 * 0.35);
        for x in 0..cols {
            for y in 0..rows {
                let index = x + y * cols;
                let r = field[index as usize] as f32;

                canvas.set_color(Color::rgb(r, r,r ));
                canvas.draw_point((x * rez) as i32, (y * rez) as i32);
            }
        }
        canvas.render();
        canvas.unbind();

        window.swap_buffers();
        window.poll_events();
        if window.should_close() {
            return;
        }
    }
}
