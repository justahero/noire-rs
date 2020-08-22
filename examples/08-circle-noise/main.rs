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
use noire::{core::{FpsTimer, Timer}, render::{OpenGLWindow, RenderWindow, Size, Window, Capability, Program, VertexArrayObject, Bindable, Drawable, Uniform, frame_buffer::copy_frame_buffer_to_image, MotionRenderPass}};

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

fn random(noise: &OpenSimplexNoise, x: f32, y: f32, t: f32) -> (f32, f32) {
    let d = dist(x, y, (WIDTH as f32) / 2.0, (HEIGHT as f32) / 2.0);
    let intensity = map(d, 0.0, RADIUS as f32, 1.0, 0.0).powf(0.75);

    let zoff = R * (TWO_PI * t).cos();
    let woff = R * (TWO_PI * t).sin();
    let xx = x + intensity * L * noise.noise4_f32(SCALE * x, SCALE * y, zoff, woff) as f32;
    let yy = y + intensity * L * noise.noise4_f32(100.0 + SCALE * x, SCALE * y, zoff, woff) as f32;

    (xx, yy)
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
        let (x, y) = random(noise, self.x, self.y, t);
        canvas.set_color(Color::rgb(1.0, 1.0, 1.0));
        canvas.draw_point(x, y);
    }
}

pub struct Example {
    /// The noise algorithm
    noise: OpenSimplexNoise,
    /// The list of stars to animate
    stars: Vec<Star>,
}

impl Example {
    pub fn new(num_stars: u32) -> Self {
        let stars = (0..num_stars).into_iter().map(|_| Star::new()).collect();

        Self {
            noise: OpenSimplexNoise::new(0),
            stars,
        }
    }

    pub fn draw_stars(&self, canvas: &mut Canvas2D, t: f32) {
        for star in &self.stars {
            star.render(canvas, &self.noise, t);
        }
        self.draw_curve(canvas, t);
    }

    pub fn draw_curve(&self, canvas: &mut Canvas2D, t: f32) {
        let m = 200;

        let points: Vec<(f32, f32)> = (0..m).into_iter().map(|i| {
            let theta: f32 = (i as f32) * TWO_PI / (m as f32);
            let x = (WIDTH as f32) / 2.0 + RADIUS * theta.sin();
            let y = (HEIGHT as f32) / 2.0 + RADIUS * theta.cos();

            random(&self.noise, x, y, t)
        }).collect();

        for i in 0..points.len() {
            let (px, py) = points[(i + 1) % points.len()];
            let (cx, cy) = points[i];
            canvas.set_color(Color::rgb(1.0, 1.0, 1.0));
            canvas.draw_line(px, py, cx, cy);
        }
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
    let mut render_pass = MotionRenderPass::new(window_size.width, window_size.height);

    let mut image_recorder = ImageSetRecorder::new("./output", NUM_FRAMES);

    // let mut field: Vec<Color> = vec![Color::rgb(0.0, 0.0, 0.0); (WIDTH * HEIGHT) as usize];
    let example = Example::new(40_000);

    loop {
        let elapsed = timer.elapsed_in_seconds() as f32;

        fps_timer.next_frame();
        println!("Frame {} - FPS {}", fps_timer.total_frames(), fps_timer.fps());

        // let size = window.get_framebuffer_size();
        window.reset_viewport();
        window.clear(0.0, 0.0, 0.0, 1.0);

        render_pass.set_render_target();
        canvas.clear(Color::BLACK);
        canvas.bind();

        let t = (fps_timer.total_frames() as f32) / (NUM_FRAMES as f32);
        example.draw_stars(&mut canvas, t);

        canvas.unbind();

        render_pass.reset();
        render_pass.draw();

        // Grab the content of the frame buffer
        /*
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
