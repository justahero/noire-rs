#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate cgmath;
extern crate gl;
extern crate glfw;
extern crate notify;

#[macro_use]
extern crate noire;

use gl::types::*;

use cgmath::{Point3, Vector3};
use cgmath::vec3;

use noire::render::{Primitive, Program, Shader, Size, Uniform, VertexBuffer, VertexArrayObject};
use noire::render::{Bindable, Drawable};
use noire::render::{Fullscreen, Pos, OpenGLWindow, RenderWindow, Window};
use noire::math::{Camera, Color};
use noire::math::*;
use noire::input::*;
use noire::input::keyboard::*;

use notify::*;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};
use std::thread;
use std::thread::JoinHandle;
use std::collections::VecDeque;

static VERTICES: [GLfloat; 8] = [-1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0];

const MAX_FPS_COUNT: u32 = 50;

fn watch_program(
    rx: &Receiver<notify::DebouncedEvent>,
    vertex_file: &str,
    fragment_file: &str,
) -> std::option::Option<Program> {
    match rx.try_recv() {
        Ok(DebouncedEvent::Write(path)) => {
            if let Ok(program) = Program::compile_from_files(&vertex_file, &fragment_file) {
                return Some(program);
            }
        }
        _ => (),
    }
    None
}

fn from_duration(d: Duration) -> f32 {
    (d.as_secs() as f64 + d.subsec_nanos() as f64 * 1e-9) as f32
}

fn main() {
    let mut window = RenderWindow::create(600, 400, "Hello This is window")
        .expect("Failed to create Render Window");

    println!("Context version: {:?}", window.window.get_context_version());

    // create shader program
    let vertex_file = String::from("./examples/03-raymarching/shaders/vertex.glsl");
    let fragment_file = String::from("./examples/03-raymarching/shaders/fragment.glsl");
    let mut program: Program = Program::compile_from_files(&vertex_file, &fragment_file).unwrap();

    // enable file watching
    let files = vec![&vertex_file, &fragment_file];
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(125)).unwrap();
    for file in &files {
        watcher.watch(&file, RecursiveMode::NonRecursive).unwrap();
    }

    let mut camera = Camera::new();
    camera
        .perspective(60.0, window.aspect(), 0.1, 80.0)
        .lookat(
            point3(0.0, 2.0, -4.5),
            point3(0.0, 2.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        )
        .set_position(point3(0.0, 2.0, 20.0));

    // create vertex data
    let vb = VertexBuffer::create(&VERTICES, 2, Primitive::TriangleStrip);
    let mut vao = VertexArrayObject::new().unwrap();
    vao.add_vb(vb);

    let mut last_pos = Pos { x: 0, y: 0 };
    let mut last_size = Size {
        width: 0,
        height: 0,
    };

    let start_time = Instant::now();
    let mut last_time = start_time;
    let mut list_frames = VecDeque::new();

    loop {
        if let Some(new_program) = watch_program(&rx, &vertex_file, &fragment_file) {
            program = new_program;
        }

        let now = Instant::now();
        let elapsed = from_duration(now.duration_since(start_time));
        let frame_elapsed = from_duration(now.duration_since(last_time));

        last_time = now;

        // update fps calulation
        list_frames.push_back(frame_elapsed);
        if list_frames.len() > (MAX_FPS_COUNT as usize) {
            list_frames.pop_front();
        }
        let fps: f32 = list_frames.iter().sum();
        let fps = 1.0 / fps * (MAX_FPS_COUNT as f32);
        // println!("FPS: {}", fps);

        window.clear(0.0, 0.0, 0.0, 1.0);
        window.clear_depth(1.0);

        camera.perspective(60.0, window.aspect(), 0.1, 100.0);
        let size = window.get_framebuffer_size();

        program.bind();
        program.uniform("u_resolution", size.into());
        program.uniform("u_aspect", camera.aspect.into());
        program.uniform("u_time", elapsed.into());
        program.uniform("u_znear", camera.znear.into());
        program.uniform("u_zfar", camera.zfar.into());
        program.uniform("u_cameraPos", camera.position.into());
        program.uniform("u_camView", Uniform::Mat4(camera.invert_view().unwrap()));
        program.uniform("u_ambientColor", color!(0.0, 0.0, 0.0).into());
        program.uniform("u_light", vec3(0.0, 20.0, 0.0).into());
        program.uniform("u_lightColor", color!(0.4, 1.0, 1.0).into());

        vao.bind();
        vao.draw();
        vao.unbind();

        program.unbind();

        // limits to 60 frames a second
        window.swap_buffers();

        window.poll_events();
        while let Some(input) = window.poll_event() {
            match input {
                Input::Press(Button::Keyboard(Key::Enter)) => {
                    if window.is_fullscreen() {
                        window.set_windowed(&last_pos, &last_size);
                    } else {
                        last_pos = window.pos();
                        last_size = window.size();
                        window.set_fullscreen(Fullscreen::Current);
                    }
                }
                Input::Pressed(Button::Keyboard(Key::Left)) => {
                    let pos = camera.position + camera.right() * -0.5;
                    camera.set_position(pos);
                }
                Input::Pressed(Button::Keyboard(Key::Right)) => {
                    let pos = camera.position + camera.right() * 0.5;
                    camera.set_position(pos);
                }
                Input::Pressed(Button::Keyboard(Key::Up)) => {
                    let pos = camera.position + camera.forward() * -0.5;
                    camera.set_position(pos);
                }
                Input::Pressed(Button::Keyboard(Key::Down)) => {
                    let pos = camera.position + camera.forward() * 0.5;
                    camera.set_position(pos);
                }
                Input::Press(Button::Keyboard(Key::Escape)) => {
                    window.close();
                }
                _ => (),
            }
        }

        if window.should_close() {
            return;
        }
    }
}
