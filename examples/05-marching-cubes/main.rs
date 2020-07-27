#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate gl;
extern crate noire;
extern crate notify;

use gl::types::*;

use noire::canvas::Canvas2D;
use noire::math::{Color, PerlinNoise, Rect};
use noire::render::{OpenGLWindow, RenderWindow, Size, Window, Capability};
use std::time::{Duration, Instant};

fn main() {
    let window_size = Size::new(900, 600);
    let mut window = RenderWindow::create(&window_size, "Hello This is window")
        .expect("Failed to create Render Window");

    window.enable(Capability::ProgramPointSize);

    let mut canvas = Canvas2D::new();
    let perlin = PerlinNoise::new(10);

    let start_time = Instant::now();
    let increment = 0.02;

    loop {
        let now = Instant::now();
        let elapsed = now.duration_since(start_time);
        let elapsed = (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9) as f32;

        let size = window.get_framebuffer_size();

        window.reset_viewport();
        window.clear(0.3, 0.3, 0.3, 1.0);
        canvas.set_color(Color::rgb(1.0, 0.0, 0.0));

        let height: f64  = size.height as f64;
        let mut xoff = 0.0;
        let mut last_y = 0.0;

        for x in 1..size.width {
            let y = height / 2.0 + height / 2.0 * perlin.gen1(xoff);

            // canvas.draw_point(x as i32, y as i32);
            canvas.draw_line((x - 1) as i32, last_y as i32, x as i32, y as i32);

            xoff += increment;
            last_y = y;
        }

        canvas.render(&size);

        window.swap_buffers();

        window.poll_events();
        if window.should_close() {
            return;
        }
    }
}
