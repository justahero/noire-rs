extern crate gl;
extern crate glfw;

mod noire;

use glfw::{Action, Context, Key};
use noire::shader::*;
use noire::program::*;

static VS_SRC: &'static str = "#version 150\n\
    in vec2 position;\n\
    void main() {\n\
       gl_Position = vec4(position, 0.0, 1.0);\n\
    }";

static FS_SRC: &'static str = "#version 150\n\
    out vec4 out_color;\n\
    void main() {\n\
       out_color = vec4(1.0, 1.0, 1.0, 1.0);\n\
    }";

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) =
        glfw.create_window(400, 300, "Hello This is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

    // load gl functions
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let vertex_shader = Shader::create(VS_SRC, ShaderType::VertexShader).unwrap();
    let pixel_shader = Shader::create(FS_SRC, ShaderType::PixelShader).unwrap();

    let _program = Program::create(vertex_shader, pixel_shader);

    window.set_key_polling(true);
    window.make_current();
    glfw.set_swap_interval(glfw::SwapInterval::None);

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
