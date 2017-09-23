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
use std::time::Instant;

/* Shader sources */
static VERT_SHADER: &'static str = "#version 330\n\
    layout(location = 0) in vec2 position;\n\
    uniform float angle;\n\
    void main() {\n\
        gl_Position = vec4(position, 0.0, 1.0);\n\
    }\n";

static FRAG_SHADER: &'static str = "#version 330\n\
    out vec4 color;\n\
    void main() {\n\
        color = vec4(1, 0.15, 0.15, 0);\n\
    }\n";

static VERTICES: [GLfloat; 8] = [-1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0];

fn main() {
    let mut glfw = glfw::init(Some(glfw::Callback {
        f: glfw_error_callback as fn(glfw::Error, String, &Cell<usize>),
        data: Cell::new(0),
    })).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::Resizable(false));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) =
        glfw.create_window(600, 400, "Hello This is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

    window.set_key_polling(true);
    window.make_current();

    // glfw.set_swap_interval(glfw::SwapInterval::None);
    glfw.set_swap_interval(glfw::SwapInterval::None);

    // load gl functions
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let vertex_shader = Shader::create(VERT_SHADER, gl::VERTEX_SHADER).unwrap();
    let pixel_shader = Shader::create(FRAG_SHADER, gl::FRAGMENT_SHADER).unwrap();
    let program = Program::create(vertex_shader, pixel_shader).unwrap();

    // initialize GL shader stuff
    let vb = VertexBuffer::create(&VERTICES, 2, gl::TRIANGLE_STRIP);
    let mut vao = VertexArrayObject::new();
    vao.add_vb(vb);

    let start_time = Instant::now();

    while !window.should_close() {
        let now = Instant::now();
        let elapsed = now.duration_since(start_time);
        let _elapsed = (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9) as f32;

        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // render square
        program.bind();
        vao.bind();
        vao.draw();
        vao.unbind();
        program.unbind();

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn glfw_error_callback(error: glfw::Error, description: String, _error_count: &Cell<usize>) {
    panic!("GL ERROR: {} - {}", error, description);
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
