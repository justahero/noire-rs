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

    let timer = Timer::now();
    let mut fps_timer = FpsTimer::now();

    let mut canvas = Canvas2D::new(600, 600);
    let noise = PerlinNoise::new(0);

    loop {
        let elapsed = timer.elapsed_in_seconds() as f32;

        fps_timer.next_frame();
        println!("FPS: {}", fps_timer.fps());

        let size = window.get_framebuffer_size();
        window.reset_viewport();
        window.clear(0.3, 0.3, 0.3, 1.0);
        // canvas.set_color(Color::rgb(1.0, 0.0, 0.0));

        let increment = 0.04;
        let mut yoff = 0.5;

        // render to canvas
        canvas.bind();
        for y in 0..size.height {
            let mut xoff = 0.0;

            for x in 0..size.width {
                let index = x + y * size.width;
                // let r = 0.5 + noise.gen2(xoff, yoff) as f32;

                xoff += increment;
            }

            yoff += increment;
        }
        canvas.unbind();

        canvas.render(&size);

        window.swap_buffers();
        window.poll_events();
        if window.should_close() {
            return;
        }
    }
}
