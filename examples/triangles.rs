#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate gl;
extern crate noire;
extern crate notify;

use gl::types::*;

use noire::render::shader::*;
use noire::render::program::*;
use noire::render::traits::*;
use noire::render::vertex::*;
use noire::render::window::RenderWindow;

use notify::*;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use std::thread;
use std::thread::JoinHandle;

static VERTICES: [GLfloat; 8] = [-1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0];

fn compile_program(
    vertex_file: &String,
    fragment_file: &String,
) -> std::result::Result<Program, String> {
    let vertex_shader;
    let fragment_shader;

    match create_shdaer_from_file(vertex_file, gl::VERTEX_SHADER) {
        Ok(shader) => vertex_shader = shader,
        Err(e) => return Err(e),
    }
    match create_shdaer_from_file(fragment_file, gl::FRAGMENT_SHADER) {
        Ok(shader) => fragment_shader = shader,
        Err(e) => return Err(e),
    }
    Program::create(vertex_shader, fragment_shader)
}

fn main() {
    let mut window = RenderWindow::create(600, 600, "Hello This is window")
        .expect("Failed to create Render Window");

    // create shader program
    let vertex_file = String::from("./examples/shaders/vertex.glsl");
    let fragment_file = String::from("./examples/shaders/fragment.glsl");
    let mut program: Program = compile_program(&vertex_file, &fragment_file).unwrap();

    // enable file watching
    let files = vec![&vertex_file, &fragment_file];
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(125)).unwrap();
    for file in &files {
        watcher.watch(&file, RecursiveMode::NonRecursive).unwrap();
    }

    // create vertex data
    let vb = VertexBuffer::create(&VERTICES, 2, gl::TRIANGLE_STRIP);
    let mut vao = VertexArrayObject::new();
    vao.add_vb(vb);

    let start_time = Instant::now();

    loop {
        // check if there is a file system event
        match rx.try_recv() {
            Ok(DebouncedEvent::Write(path)) => {
                match compile_program(&vertex_file, &fragment_file) {
                    Ok(new_program) => {
                        program = new_program;
                    }
                    Err(e) => println!("Failed to set new program: {:?}", e),
                }
            }
            _ => (),
        }

        let now = Instant::now();
        let elapsed = now.duration_since(start_time);
        let elapsed = (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9) as f32;

        window.clear(0.3, 0.3, 0.3, 1.0);

        let (width, height) = window.get_size();

        program.bind();
        program.uniform2f("u_resolution", width as f32, height as f32);
        program.uniform1f("u_time", elapsed as f32);

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
