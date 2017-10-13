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
use glfw::Key;

use cgmath::{Point3, Vector3};
use cgmath::vec3;

use noire::render::shader::*;
use noire::render::program::*;
use noire::render::traits::*;
use noire::render::vertex::*;
use noire::render::window::RenderWindow;
use noire::math::camera::*;
use noire::math::color::*;

use notify::*;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};
use std::thread;
use std::thread::JoinHandle;

static VERTICES: [GLfloat; 8] = [-1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0];

fn watch_program(
    rx: &Receiver<notify::DebouncedEvent>,
    vertex_file: &String,
    fragment_file: &String,
) -> std::option::Option<Program> {
    match rx.try_recv() {
        Ok(DebouncedEvent::Write(path)) => {
            match Program::compile_from_files(&vertex_file, &fragment_file) {
                Ok(program) => return Some(program),
                Err(e) => (),
            }
        }
        _ => (),
    }
    None
}

fn keypress_callback(key: Key) {
    // TODO
}

fn main() {
    let mut window = RenderWindow::create(600, 600, "Hello This is window")
        .expect("Failed to create Render Window");

    // create shader program
    let vertex_file = String::from("./examples/raymarching/shaders/vertex.glsl");
    let fragment_file = String::from("./examples/raymarching/shaders/fragment.glsl");
    let mut program: Program = Program::compile_from_files(&vertex_file, &fragment_file).unwrap();

    // enable file watching
    let files = vec![&vertex_file, &fragment_file];
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(125)).unwrap();
    for file in &files {
        watcher.watch(&file, RecursiveMode::NonRecursive).unwrap();
    }

    let mut camera = Camera::new();
    camera.perspective(60.0, 1.0, 0.1, 100.0);
    camera.lookat(
        Point3 {
            x: 0.0,
            y: 2.0,
            z: -4.5,
        },
        Point3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        },
        vec3(0.0, 1.0, 0.0),
    );

    // set input callbacks
    window.set_keypress_callback(keypress_callback);

    // create vertex data
    let vb = VertexBuffer::create(&VERTICES, 2, gl::TRIANGLE_STRIP);
    let mut vao = VertexArrayObject::new();
    vao.add_vb(vb);

    let start_time = Instant::now();

    loop {
        match watch_program(&rx, &vertex_file, &fragment_file) {
            Some(new_program) => program = new_program,
            None => (),
        }

        let now = Instant::now();
        let elapsed = now.duration_since(start_time);
        let elapsed = (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9) as f32;

        window.clear(0.3, 0.3, 0.3, 1.0);

        let (width, height) = window.get_framebuffer_size();

        program.bind();
        program.uniform("u_resolution", Uniform::Size(width as f32, height as f32));
        program.uniform("u_time", Uniform::Float(elapsed));
        program.uniform("u_ambientColor", Uniform::Color(color!(0.0)));
        program.uniform("u_light", Uniform::Float3(0.0, 20.0, 0.0));
        program.uniform("u_lightColor", Uniform::Color(color!(0.4, 1.0, 1.0)));
        program.uniform("u_aspect", Uniform::Float(camera.aspect));
        program.uniform("u_znear", Uniform::Float(camera.znear));
        program.uniform("u_zfar", Uniform::Float(camera.zfar));
        program.uniform("u_cameraPos", Uniform::Point3(camera.position));
        program.uniform("u_camView", Uniform::Mat4(camera.invert_view().unwrap()));

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
