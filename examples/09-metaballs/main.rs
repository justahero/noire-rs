#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate gl;
extern crate noire;
extern crate notify;

use gl::types::*;

use noire::core::{FpsTimer, Timer};
use noire::render::{Bindable, Drawable, Primitive, Program, Shader, VertexArrayObject};
use noire::render::{IndexBuffer, VertexBuffer};
use noire::{math::{random_f32, Vector2}, render::{OpenGLWindow, RenderWindow, Size, Window, Uniform}};

use std::time::{Duration, Instant};
use utils::app_dir;
use cgmath::Vector3;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

const NUM_BALLS: u32 = 8;
const NUM_FRAMES: u32 = 360;

#[derive(Debug)]
pub struct Ball {
    /// position of the ball
    pub position: Vector2,
    /// Velocity of the ball
    pub velocity: Vector2,
    /// radius of the ball
    pub radius: f32,
}

impl Ball {
    pub fn new(x: f32, y: f32, velx: f32, vely: f32, radius: f32) -> Self {
        Self {
            position: Vector2::new(x, y),
            velocity: Vector2::new(velx, vely),
            radius,
        }
    }

    pub fn vec3(&self) -> [f32; 3] {
        [self.position.x, self.position.y, self.radius]
    }

    pub fn update(&mut self) {
        self.position += self.velocity;

        if self.position.x < 0.0 || self.position.x > WIDTH as f32 {
            self.velocity.x *= -1.0;
        }

        if self.position.y < 0.0 || self.position.y > HEIGHT as f32 {
            self.velocity.y *= -1.0;
        }
    }
}

fn main() {
    let app_dir = app_dir().unwrap();
    let window_size = Size::new(WIDTH, HEIGHT);
    let mut window = RenderWindow::create(&window_size, "Metaballs").unwrap();

    // create shader program
    let vertex_file = app_dir.join("examples/09-metaballs/shaders/vertex.glsl");
    let fragment_file = app_dir.join("examples/09-metaballs/shaders/fragment.glsl");
    let mut program: Program = Program::compile_from_files(&vertex_file, &fragment_file).unwrap();

    // create vertex data
    let mut vao = VertexArrayObject::screen_rect();

    let mut balls: Vec<Ball> = (0..NUM_BALLS).into_iter().map(|_| {
        let pos = Vector2::new(random_f32(WIDTH as f32), random_f32(HEIGHT as f32));
        let vel = Vector2::random() * (2.0 + random_f32(2.5));

        let radius = 5.0 + random_f32(15.0);
        Ball::new(
            pos.x, pos.y,
            vel.x, vel.y,
            radius,
        )
    }).collect();

    let timer = Timer::now();
    let mut fps_timer = FpsTimer::now();

    loop {
        let elapsed = timer.elapsed_in_seconds() as f32;

        fps_timer.next_frame();
        // println!("Frame {} - FPS: {}", fps_timer.total_frames(), fps_timer.fps());

        window.reset_viewport();
        window.clear(0.0, 0.0, 0.0, 1.0);

        // animate the points
        let t = (fps_timer.total_frames() as f32) / (NUM_FRAMES as f32);
        for ball in &mut balls {
            ball.update();
        }

        // render all meta balls
        program.bind();
        let vertices = balls
            .iter()
            .map(|b| b.vec3())
            .collect::<Vec<[f32; 3]>>();
        program.uniform("u_balls[0]", Uniform::Vec3Array(vertices));

        vao.bind();
        vao.draw();
        vao.unbind();

        program.unbind();

        window.swap_buffers();
        window.poll_events();
        if window.should_close() {
            return;
        }
    }
}
