extern crate glfw;

use std::cell::Cell;

pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::Window,
}

fn glfw_error_callback(error: glfw::Error, description: String, _error_count: &Cell<usize>) {
    panic!("GL ERROR: {} - {}", error, description);
}

impl Window {
    pub fn create(title: &str) -> Window {
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

        Window {
            glfw: glfw,
            window: window,
        }
    }
}
