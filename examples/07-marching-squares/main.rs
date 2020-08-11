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
use noire::math::{Color, PerlinNoise, random_f32, Rect, Vector2};
use noire::{core::{FpsTimer, Timer}, render::{OpenGLWindow, RenderWindow, Size, Window, Capability, Program, VertexArrayObject, Bindable, Drawable, Uniform}};
use std::time::Instant;
use cgmath::{Vector3, Matrix3, InnerSpace, Rad, Matrix4, Vector4, Deg};

fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
    (1.0 - t) * v0 + t * v1
}

fn get_state(a: i32, b: i32, c: i32, d: i32) -> i32 {
    a * 8 + b * 4 + c * 2 + d
}

fn line(canvas: &mut Canvas2D, l: &Vector2, r: &Vector2) {
    canvas.draw_line(l.x, l.y, r.x, r.y);
}

fn main() {
    let window_size = Size::new(800, 800);

    let mut window = RenderWindow::create(&window_size, "Hello This is window").unwrap();
    window.enable(Capability::ProgramPointSize);

    let timer = Timer::now();
    let mut fps_timer = FpsTimer::now();

    let mut canvas = Canvas2D::new(800, 800);
    let noise = OpenSimplexNoise::new(0);

    let rez = 6.0;
    let cols = 1 + canvas.width / (rez as u32);
    let rows = 1 + canvas.height / (rez as u32);

    let increment = 0.04;
    let zincrement = 0.0025;
    let mut zoff = 0.0;

    let mut field: Vec<f32> = vec![0.0; (cols * rows) as usize];

    loop {
        let elapsed = timer.elapsed_in_seconds() as f32;

        fps_timer.next_frame();
        println!("FPS: {}", fps_timer.fps());

        let size = window.get_framebuffer_size();
        window.reset_viewport();
        window.clear(0.4, 0.4, 0.4, 1.0);

        // render to canvas
        canvas.bind();

        // render all points
        canvas.set_pointsize(rez * 0.35);
        let mut xoff = 0.0;
        for x in 0..cols {
            xoff += increment;
            let mut yoff = 0.0;
            for y in 0..rows {
                let index = x + y * cols;

                let r = noise.noise4_classic(xoff, yoff, 0.0, zoff) as f32;
                field[index as usize] = r;

                canvas.set_color(Color::rgb(r, r,r ));
                canvas.draw_rect(x as f32 * rez, y as f32 * rez, x as f32 * rez + rez, y as f32 * rez + rez);

                yoff += increment;
            }
        }
        zoff += zincrement;

        // render all iso lines, the contour
        canvas.set_color(Color::rgb(1.0, 1.0, 1.0));

        for i in 0..(cols - 1) {
            for j in 0..(rows - 1) {
                let index = i + j * cols;
                let x = (i as f32) * rez;
                let y = (j as f32) * rez;

                let a_val = field[index as usize] + 1.0;
                let b_val = field[(index + 1) as usize] + 1.0;
                let c_val = field[(index + cols + 1) as usize] + 1.0;
                let d_val = field[(index + cols) as usize] + 1.0;

                let a = Vector2::new(x + rez * 0.5, y            );
                let b = Vector2::new(x + rez      , y + rez * 0.5);
                let c = Vector2::new(x + rez * 0.5, y + rez      );
                let d = Vector2::new(x            , y + rez * 0.5);

                let state = get_state(
                    field[index as usize].ceil() as i32,
                    field[(index + 1) as usize].ceil() as i32,
                    field[(index + cols + 1) as usize].ceil() as i32,
                    field[(index + cols) as usize].ceil() as i32,
                );

                let amt = (1.0 - a_val) / (b_val - a_val);
                let a = Vector2::new(lerp(x, x + rez, amt), y);

                let amt = (1.0 - b_val) / (c_val - b_val);
                let b = Vector2::new(x + rez, lerp(y, y + rez, amt));

                let amt = (1.0 - d_val) / (c_val - d_val);
                let c = Vector2::new(lerp(x, x + rez, amt), y + rez);

                let amt = (1.0 - a_val) / (d_val - a_val);
                let d = Vector2::new(x, lerp(y, y + rez, amt));

                match state {
                    1 => {line(&mut canvas, &c, &d); },
                    2 => {line(&mut canvas, &b, &c); },
                    3 => {line(&mut canvas, &b, &d); },
                    4 => {line(&mut canvas, &a, &b); },
                    5 => {line(&mut canvas, &a, &d); line(&mut canvas, &b, &c); },
                    6 => {line(&mut canvas, &a, &c); },
                    7 => {line(&mut canvas, &a, &d); },
                    8 => {line(&mut canvas, &a, &d); },
                    9 => {line(&mut canvas, &a, &c); },
                    10 => {line(&mut canvas, &a, &b); line(&mut canvas, &c, &d); },
                    11 => {line(&mut canvas, &a, &b); },
                    12 => {line(&mut canvas, &b, &d); },
                    13 => {line(&mut canvas, &b, &c); },
                    14 => {line(&mut canvas, &c, &d); },
                    _ => (),
                };
            }
        }

        canvas.unbind();

        window.swap_buffers();
        window.poll_events();
        if window.should_close() {
            return;
        }
    }
}
