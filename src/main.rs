extern crate gl;
extern crate glfw;

mod noire;

use glfw::{Action, Context, Key};
use gl::types::*;

use noire::shader::*;
use noire::program::*;
use noire::traits::*;
use noire::vertex::*;

use std::cell::Cell;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::time::Instant;

static VS_SRC: &'static str = r##"
varying vec2 position;
void main() {
   gl_Position = vec4(position, 0.0, 1.0);
}
"##;

static FS_SRC: &'static str = r##"
void main() {
   gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
}
"##;

static VERTICES: [GLfloat; 8] = [-0.5, 0.5, -0.5, -0.5, 0.5, 0.5, 0.5, -0.5];

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) =
        glfw.create_window(600, 400, "Hello This is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::Resizable(false));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    window.set_key_polling(true);
    window.make_current();

    // glfw.set_swap_interval(glfw::SwapInterval::None);
    glfw.set_swap_interval(glfw::SwapInterval::None);

    // load gl functions
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let vertex_shader = Shader::create(VS_SRC, gl::VERTEX_SHADER).unwrap();
    let pixel_shader = Shader::create(FS_SRC, gl::FRAGMENT_SHADER).unwrap();
    let program = Program::create(vertex_shader, pixel_shader).unwrap();

    // initialize GL shader stuff
    let mut vao = 0;
    let mut vbo = 0;

    let start_time = Instant::now();

    // Create a Vertex Buffer Object and copy the vertex data to it
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            mem::size_of_val(&VERTICES) as GLsizeiptr,
            mem::transmute(&VERTICES[0]),
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    // create VertexArray Object
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    while !window.should_close() {
        let now = Instant::now();
        let elapsed = now.duration_since(start_time);
        let _elapsed = (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9) as f32;

        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // render square
            gl::UseProgram(program.id);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 3);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }

        window.swap_buffers();

        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }

    // clean up
    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
    }
}

fn glfw_error_callback(error: glfw::Error, description: String, _error_count: &Cell<usize>) {
    println!("GL ERROR: {} - {}", error, description);
    // TODO
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
