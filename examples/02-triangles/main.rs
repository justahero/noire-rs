#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate gl;
extern crate noire;
extern crate notify;

use gl::types::*;

use noire::render::Primitive;
use noire::render::shader::*;
use noire::render::program::*;
use noire::render::traits::*;
use noire::render::vertex::*;
use noire::render::vertex_buffer::*;
use noire::render::index_buffer::*;
use noire::render::window::{OpenGLWindow,RenderWindow,Window};

use notify::*;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use std::thread;
use std::thread::JoinHandle;

static VERTICES: [GLfloat; 8] = [-1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0];
static INDICES: [GLuint; 6] = [0, 1, 2, 2, 3, 1];

fn main() {
    let mut window = RenderWindow::create(600, 600, "Hello This is window")
        .expect("Failed to create Render Window");

    // create shader program
    let vertex_file = String::from("./examples/02-triangles/shaders/vertex.glsl");
    let fragment_file = String::from("./examples/02-triangles/shaders/fragment.glsl");
    let mut program: Program = Program::compile_from_files(&vertex_file, &fragment_file).unwrap();

    // enable file watching
    let files = vec![&vertex_file, &fragment_file];
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(125)).unwrap();
    for file in &files {
        watcher.watch(&file, RecursiveMode::NonRecursive).unwrap();
    }

    // create vertex data
    let vb = VertexBuffer::create(&VERTICES, 2, Primitive::TriangleStrip);
    let ib = IndexBuffer::create(&INDICES);

    let mut vao = VertexArrayObject::new();
    vao.add_vb(vb);
    vao.add_ib(ib);

    let start_time = Instant::now();

    loop {
        // check if there is a file system event
        match rx.try_recv() {
            Ok(DebouncedEvent::Write(path)) => {
                match Program::compile_from_files(&vertex_file, &fragment_file) {
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

        let size = window.get_framebuffer_size();

        program.bind();
        // is there a way to use deref coercion to not specify Uniform type?
        program.uniform("u_resolution", size.into());
        program.uniform("u_time", elapsed.into());

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