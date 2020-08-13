#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate cgmath;
extern crate gl;
extern crate noire;
extern crate notify;
extern crate opensimplex;

use gl::types::*;

use opensimplex::OpenSimplexNoise;
use noire::canvas::Canvas2D;
use noire::math::{Color, random_f32, Rect, Vector2, map};
use noire::{core::{FpsTimer, Timer}, render::{OpenGLWindow, RenderWindow, Size, Window, Capability, Program, VertexArrayObject, Bindable, Drawable, Uniform, frame_buffer::copy_frame_buffer_to_image}};

use std::{path::Path, time::Instant, ffi::c_void, fs::File, f32::consts::PI};
use cgmath::{Vector3, Matrix3, InnerSpace, Rad, Matrix4, Vector4, Deg};
use image::{RgbImage, ImageBuffer, DynamicImage, ImageFormat};
use utils::ImageSetRecorder;

static TWO_PI: f32 = 2.0 * PI;
static RADIUS: f32 = 280.0;
static NUM_FRAMES: u32 = 180;
const WIDTH: u32 = 640;
const HEIGHT: u32 = 640;

static ANGLE: f32 = 0.4;
static R: f32 = 1.05;
static L: f32 = 200.0;
static SCALE: f32 = 0.008;

fn dist(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

fn line(canvas: &mut Canvas2D, l: &Vector2, r: &Vector2) {
    canvas.draw_line(l.x, l.y, r.x, r.y);
}

struct Star {
    r: f32,
    the: f32,
    x: f32,
    y: f32,
}

impl Star {
    pub fn new() -> Star {
        let r = random_f32(1.0).sqrt() * RADIUS;
        let the = random_f32(TWO_PI);
        let x = (HEIGHT as f32) / 2.0 + r * the.sin();
        let y = (WIDTH as f32) / 2.0 + r * the.cos();

        Self {
            r, the, x, y
        }
    }

    // render the star
    pub fn render(&self, canvas: &mut Canvas2D, noise: &OpenSimplexNoise, t: f32) {
        let d = dist(self.x, self.y, (WIDTH / 2) as f32, (HEIGHT / 2) as f32);
        let intensity = map(d, 0.0, RADIUS as f32, 1.0, 0.0).powf(0.75);

        let zoff = R * (TWO_PI * t).cos();
        let woff = R * (TWO_PI * t).sin();
        let xx = self.x + intensity * L * noise.noise4_f32(SCALE * self.x, SCALE * self.y, zoff, woff) as f32;
        let yy = self.y + intensity * L * noise.noise4_f32(100.0 + SCALE * self.x, SCALE * self.y, zoff, woff) as f32;

        canvas.set_color(Color::rgb(1.0, 1.0, 1.0));
        canvas.draw_point(xx, yy);
    }
}

/// Implements the following algorithm (at least it's the goal)
/// https://gist.github.com/Bleuje/ae3662d67bea2e24092d64efe022ed4c#file-noisetraj-pde
fn main() {
    let window_size = Size::new(WIDTH, HEIGHT);

    let mut window = RenderWindow::create(&window_size, "Hello This is window").unwrap();
    window.enable(Capability::ProgramPointSize);

    let timer = Timer::now();
    let mut fps_timer = FpsTimer::now();

    let mut canvas = Canvas2D::new(window_size.width, window_size.height);
    let noise = OpenSimplexNoise::new(0);

    let mut image_recorder = ImageSetRecorder::new("./output", NUM_FRAMES);

    loop {
        let elapsed = timer.elapsed_in_seconds() as f32;

        fps_timer.next_frame();
        println!("Frame {} - FPS {}", fps_timer.total_frames(), fps_timer.fps());

        // let size = window.get_framebuffer_size();
        window.reset_viewport();
        window.clear(0.4, 0.4, 0.4, 1.0);

        // render to canvas
        canvas.bind();



        canvas.unbind();

        /*
        // Grab the content of the frame buffer
        if !image_recorder.complete() {
            let image = copy_frame_buffer_to_image(window_size.width, window_size.height).into_rgb();
            image_recorder.save_image(image).expect("Add Frame failed");
        }
        */

        window.swap_buffers();
        window.poll_events();
        if window.should_close() {
            return;
        }
    }
}
