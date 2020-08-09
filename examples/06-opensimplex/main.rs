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
    let window_size = Size::new(800, 800);
    let mut window = RenderWindow::create(&window_size, "Hello This is window")
        .expect("Failed to create Render Window");

    let timer = Timer::now();
    let mut fps_timer = FpsTimer::now();

    let mut canvas = Canvas2D::new(800, 800);
    let opensimplex = OpenSimplexNoise::new(0);

    let mut zoff = 0.0;

    loop {
        let elapsed = timer.elapsed_in_seconds() as f32;
        fps_timer.next_frame();
        println!("FPS: {}", fps_timer.fps());

        let size = window.get_framebuffer_size();
        window.reset_viewport();
        window.clear(0.3, 0.3, 0.3, 1.0);
        canvas.set_color(Color::rgb(1.0, 0.0, 0.0));
        canvas.bind();

        let increment = 0.04;
        let mut yoff = 0.5;

        // render OpenSimplex noise
        for y in 0..size.height / 4 {
            let mut xoff = 0.0;

            for x in 0..size.width / 4 {
                let index = x + y * size.width;
                let r = 0.5 + (opensimplex.noise4_classic(xoff, yoff, zoff, 0.0) / 2.0) as f32;

                canvas.set_color(Color::rgb(r, r, r));
                canvas.draw_rect((x * 4) as f32, (y * 4) as f32, ((x + 1) * 4) as f32, ((y + 1) * 4) as f32);

                xoff += increment;
            }

            yoff += increment;
        }

        zoff += 0.01;

        canvas.render();
        canvas.unbind();

        window.swap_buffers();
        window.poll_events();
        if window.should_close() {
            return;
        }
    }
}
