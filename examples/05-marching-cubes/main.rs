#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate gl;
extern crate noire;
extern crate notify;

use gl::types::*;

use noire::canvas::Canvas2D;
use noire::math::{Color, Rect};
use noire::render::{OpenGLWindow, RenderWindow, Size, Window, Capability};
use std::time::{Duration, Instant};

fn main() {
    let window_size = Size::new(1000, 600);
    let mut window = RenderWindow::create(&window_size, "Hello This is window")
        .expect("Failed to create Render Window");

    window.enable(Capability::ProgramPointSize);

    let mut canvas = Canvas2D::new();

    let start_time = Instant::now();

    loop {
        let now = Instant::now();
        let elapsed = now.duration_since(start_time);
        let elapsed = (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9) as f32;

        let window_size = window.get_size();
        let framebuffer_size = window.get_framebuffer_size();

        window.reset_viewport();
        window.clear(0.3, 0.3, 0.3, 1.0);

        canvas.set_color(Color::rgb(1.0, 0.0, 0.0));
        canvas.draw_line(0, 0, 300, 300);
        canvas.draw_line(0, 50, 350, 350);
        canvas.draw_line(0, 100, 400, 400);
        canvas.draw_line(0, 150, 450, 450);

        canvas.set_color(Color::rgb(0.0, 1.0, 0.2));
        canvas.draw_rect(100, 350, 200, 500);
        canvas.draw_rect(250, 400, 350, 550);

        canvas.set_color(Color::rgb(1.0, 1.0, 0.0));
        canvas.set_pointsize(5.0);
        canvas.draw_point(400, 250);
        canvas.draw_point(400, 300);

        canvas.render(&framebuffer_size);

        window.swap_buffers();

        window.poll_events();
        if window.should_close() {
            return;
        }
    }
}
