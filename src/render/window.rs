#![allow(unused_variables)]
use gl;
use std::cell::Cell;
use std::sync::mpsc::Receiver;

use glfw;
use glfw::{Context, Glfw, Error, Window, WindowEvent};

pub struct RenderWindow {
    pub glfw: Glfw,
    pub window: Window,
    events: Receiver<(f64, WindowEvent)>,
}

fn glfw_error_callback(error: Error, description: String, _error_count: &Cell<usize>) {
    panic!("GL ERROR: {} - {}", error, description);
}

// make struct function
pub fn set_fullscreen(glfw: &mut Glfw, window: &mut Window) {
    glfw.with_primary_monitor_mut(|_: &mut _, m: Option<&glfw::Monitor>| {
        let monitor = m.unwrap();
        let mode: glfw::VidMode = monitor.get_video_mode().unwrap();

        window.set_monitor(
            glfw::WindowMode::FullScreen(&monitor),
            0,
            0,
            mode.width,
            mode.height,
            Some(mode.refresh_rate),
        );
        println!(
            "{}x{} fullscreen enabled at {}Hz on monitor {}",
            mode.width,
            mode.height,
            mode.refresh_rate,
            monitor.get_name()
        );
    });
}

impl RenderWindow {
    pub fn create(width: u32, height: u32, title: &str) -> Result<RenderWindow, String> {
        let mut glfw = match glfw::init(Some(glfw::Callback {
            f: glfw_error_callback as fn(glfw::Error, String, &Cell<usize>),
            data: Cell::new(0),
        })) {
            Ok(glfw) => glfw,
            Err(_) => return Err("Failed to initialize GLFW".to_string()),
        };

        glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
        glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
        glfw.window_hint(glfw::WindowHint::Resizable(false));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        let (mut window, events) =
            match glfw.create_window(width, height, title, glfw::WindowMode::Windowed) {
                Some(result) => result,
                _ => return Err("Failed to create GLFW Window".to_string()),
            };

        window.set_key_polling(true);
        window.make_current();

        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        // load gl functions
        gl::load_with(|s| window.get_proc_address(s) as *const _);

        Ok(RenderWindow {
            glfw: glfw,
            window: window,
            events: events,
        })
    }

    pub fn aspect(&self) -> f32 {
        let (width, height) = self.window.get_size();
        width as f32 / height as f32
    }

    pub fn close(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn get_framebuffer_size(&self) -> (i32, i32) {
        self.window.get_framebuffer_size()
    }

    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn clear_depth(&self, value: f32) {
        unsafe {
            gl::ClearDepthf(value);
            gl::Clear(gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Key(key, _, glfw::Action::Press, mods) => {}
                WindowEvent::Key(key, _, glfw::Action::Release, mods) => {}
                _ => {}
            }
        }
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers()
    }
}
