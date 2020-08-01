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
    let window_size = Size::new(800, 800);
    let mut window = RenderWindow::create(&window_size, "Hello This is window")
        .expect("Failed to create Render Window");

    let mut canvas = Canvas2D::new();
    let perlin = PerlinNoise::new(10);

    let start_time = Instant::now();
    let increment = 0.01;

    loop {
        let now = Instant::now();
        let elapsed = now.duration_since(start_time);
        let elapsed = (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9) as f32;

        let size = window.get_framebuffer_size();

        window.reset_viewport();
        window.clear(0.3, 0.3, 0.3, 1.0);
        canvas.set_color(Color::rgb(1.0, 0.0, 0.0));

        let increment = 0.04;
        let mut yoff = 0.5;

        // Render 4x4 pixel rects to create less geometry
        for y in 0..size.height / 4 {
            let mut xoff = 0.0;

            for x in 0..size.width / 4 {
                let index = x + y * size.width;

                let r = perlin.gen2(xoff, yoff) as f32;

                canvas.set_color(Color::rgb(r, r, r));

                canvas.draw_rect(
                    (x * 4) as i32,
                    (y * 4) as i32,
                    ((x + 1) * 4) as i32,
                    ((y + 1) * 4) as i32
                );

                xoff += increment;
            }
            yoff += increment;
        }

        canvas.render(&size);

        window.swap_buffers();

        window.poll_events();
        if window.should_close() {
            return;
        }
    }
}
