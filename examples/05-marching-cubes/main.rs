#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate gl;
extern crate noire;
extern crate notify;
extern crate rand;

use gl::types::*;

use noire::canvas::Canvas2D;
use noire::math::Rect;
use noire::render::{OpenGLWindow, RenderWindow, Size, Window};
use rand::prelude::*;
use std::time::{Duration, Instant};

fn main() {
    let window_size = Size::new(1000, 600);
    let mut window = RenderWindow::create(&window_size, "Hello This is window")
        .expect("Failed to create Render Window");

    let mut canvas = Canvas2D::new();
    let mut rng = rand::thread_rng();

    let start_time = Instant::now();

    loop {
        let now = Instant::now();
        let elapsed = now.duration_since(start_time);
        let elapsed = (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9) as f32;

        let window_size = window.get_size();
        let framebuffer_size = window.get_framebuffer_size();

        window.reset_viewport();
        window.clear(0.3, 0.3, 0.3, 1.0);

        canvas.draw_line(0, 0, 300, 300);

        canvas.render(&framebuffer_size);

        window.swap_buffers();

        window.poll_events();
        if window.should_close() {
            return;
        }
    }
}
