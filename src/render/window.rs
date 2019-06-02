#![allow(unused_variables)]
#![allow(unused_imports)]

use std::cell::Cell;
use std::ffi::*;
use std::sync::mpsc::Receiver;
use std::collections::VecDeque;

use gl;
use glfw;
use glfw::{Context, Glfw, Error, WindowEvent};

use input::{Button, Input};
use input::keyboard::Key;
use super::Size;

/// Struct to provide coordinates
pub struct Pos<T> {
    /// x coordinate
    pub x: T,
    /// y coordinate
    pub y: T,
}

/// Struct to define fullscreen mode
pub enum Fullscreen {
    /// Use current screen resolution
    Current,
    /// Specify dimensions directly
    Size(Size<u32>),
}

/// Trait that handles a Window
///
/// The basic behavior of a Window is defined here
pub trait Window {
    /// Returns the size of the window
    fn size(&self) -> Size<u32>;
    /// Returns the position of the window
    fn pos(&self) -> Pos<i32>;
    /// Closes the window
    fn close(&mut self);
    /// Display the window
    fn show(&mut self);
    /// Returns true if the window should be closed
    fn should_close(&self) -> bool;
    /// Polls an input event from window
    fn poll_event(&mut self) -> Option<Input>;
}

/// Trait that defines an OpenGL specific Window
pub trait OpenGLWindow: Window {
    /// Basic setup of an OpenGLWindow
    fn setup(&self);
    /// Returns true if this window is the current one
    fn is_current(&self) -> bool;
    /// Make this window the current one
    fn make_current(&mut self);
    /// Returns true if window is running in fullscreen mode
    fn is_fullscreen(&self) -> bool;
    /// Set the Window into fullscreen mode
    fn set_fullscreen(&mut self, mode: Fullscreen);
    /// set windowed mode
    fn set_windowed(&mut self, pos: &Pos<i32>, size: &Size<u32>);
    /// Return the size of the frame buffer
    fn get_framebuffer_size(&self) -> Size<u32>;
    /// Clear window to a color
    fn clear(&self, r: f32, g: f32, b: f32, a: f32);
    /// Clear the depth buffer to a value
    fn clear_depth(&self, value: f32);
    /// Swaps frame buffer and displays content
    fn swap_buffers(&mut self);
}

/// Struct that defines a window to render graphics
pub struct RenderWindow {
    /// GL instance
    pub glfw: Glfw,
    /// GL Window instance
    pub window: glfw::Window,
    /// Listener of new window events from glfw
    events: Receiver<(f64, WindowEvent)>,
    /// vector of input events
    input_events: VecDeque<Input>,
    /// vector of pressed keys / buttons
    pressed_buttons: VecDeque<(Button, u32)>,
}

/// callback function to report error
fn glfw_error_callback(error: Error, description: String, _error_count: &Cell<usize>) {
    panic!("GL ERROR: {} - {}", error, description);
}

/// Conversion helper to get Size struct
impl From<glfw::VidMode> for Size<u32> {
    fn from(video_mode: glfw::VidMode) -> Self {
        Size { width: video_mode.width, height: video_mode.height }
    }
}

/// make struct function?
pub fn set_fullscreen(glfw: &mut Glfw, window: &mut glfw::Window, mode: Fullscreen) {
    glfw.with_primary_monitor_mut(|_: &mut _, m: Option<&glfw::Monitor>| {
        let monitor = m.unwrap();
        let video_mode: glfw::VidMode = monitor.get_video_mode().unwrap();

        let refresh_rate = video_mode.refresh_rate;

        let new_size = match mode {
            Fullscreen::Current => video_mode.into(),
            Fullscreen::Size(size) => size,
        };

        window.set_monitor(
            glfw::WindowMode::FullScreen(&monitor),
            0,
            0,
            new_size.width,
            new_size.height,
            Some(refresh_rate),
        );
        println!(
            "{}x{} fullscreen enabled at {}Hz on monitor {}",
            new_size.width,
            new_size.height,
            refresh_rate,
            monitor.get_name().unwrap()
        );
    });
}

/// Struct to render a window
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
        glfw.window_hint(glfw::WindowHint::Resizable(true));
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
        window.show();

        // enable Vertical Sync
        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        // load gl functions
        gl::load_with(|s| window.get_proc_address(s) as *const _);

        // create instance and initialize the window
        let render_window = RenderWindow {
            glfw: glfw,
            window: window,
            events: events,
            input_events: VecDeque::new(),
            pressed_buttons: VecDeque::new(),
        };
        render_window.setup();

        Ok(render_window)
    }

    pub fn aspect(&self) -> f32 {
        let (width, height) = self.window.get_size();
        width as f32 / height as f32
    }

    /// Poll all events from glfw instance
    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Key(key, _, glfw::Action::Press, mods) => {
                    let button = Button::Keyboard(key.into());
                    self.pressed_buttons.push_back((button, 0))
                }
                WindowEvent::Key(key, _, glfw::Action::Release, mods) => {
                    let button = Button::Keyboard(key.into());
                    if let Some(index) = self.pressed_buttons.iter().position(|&(b, _)| b == button) {
                        self.pressed_buttons.remove(index);
                    }
                }
                _ => (),
            }
        }
        // check if keys are presed and queue input events
        for &(button, count) in &self.pressed_buttons {
            match count {
                0 => {
                    self.input_events.push_back(Input::Press(button));
                }
                _ => {
                    self.input_events.push_back(Input::Pressed(button));
                }
            }
        }
        // update all button keys
        for tuple in self.pressed_buttons.iter_mut() {
            *tuple = (tuple.0, tuple.1 + 1);
        }
    }
}

/// Implement Window functions
impl Window for RenderWindow {
    fn pos(&self) -> Pos<i32> {
        let (x, y) = self.window.get_pos();
        Pos { x, y }
    }

    fn size(&self) -> Size<u32> {
        let (width, height) = self.window.get_size();
        Size {
            width: width as u32,
            height: height as u32,
        }
    }

    fn show(&mut self) {
        self.window.show();
    }

    fn close(&mut self) {
        self.window.set_should_close(true);
    }

    fn should_close(&self) -> bool {
        self.window.should_close()
    }

    fn poll_event(&mut self) -> Option<Input> {
        self.input_events.pop_front()
    }
}

/// OpenGL version of the render window
impl OpenGLWindow for RenderWindow {
    fn setup(&self) {
        unsafe {
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::DEPTH_TEST);
        }
    }

    /// Returns true if the window is the current one
    fn is_current(&self) -> bool {
        self.window.is_current()
    }
    /// Sets the window as current
    fn make_current(&mut self) {
        self.window.make_current()
    }
    /// Returns true if Window is running in fullscreen
    fn is_fullscreen(&self) -> bool {
        self.window.with_window_mode(|mode| match mode {
            glfw::WindowMode::Windowed => false,
            glfw::WindowMode::FullScreen(_) => true,
        })
    }
    /// Set the Window into fullscreen mode
    fn set_fullscreen(&mut self, mode: Fullscreen) {
        self.glfw.set_swap_interval(glfw::SwapInterval::Sync(0));
        set_fullscreen(&mut self.glfw, &mut self.window, mode);
    }
    /// Set the Window into Windowed mode
    fn set_windowed(&mut self, pos: &Pos<i32>, size: &Size<u32>) {
        self.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        self.window.set_monitor(
            glfw::WindowMode::Windowed,
            pos.x,
            pos.y,
            size.width,
            size.height,
            None,
        );
    }
    /// Return the size of the frame buffer
    fn get_framebuffer_size(&self) -> Size<u32> {
        let (width, height) = self.window.get_framebuffer_size();
        Size {
            width: width as u32,
            height: height as u32,
        }
    }
    /// Clear the frame buffer of the window to a color
    fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
    /// Clear the depth buffer to a value
    fn clear_depth(&self, value: f32) {
        unsafe {
            gl::ClearDepthf(value);
            gl::Clear(gl::DEPTH_BUFFER_BIT);
        }
    }
    /// Swap frame buffer, update with content
    fn swap_buffers(&mut self) {
        self.window.swap_buffers()
    }
}
