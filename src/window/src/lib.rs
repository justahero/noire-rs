pub mod event;
pub mod window;
pub mod windows;

use app::prelude::App;
pub use event::*;
pub use window::*;
pub use windows::*;

pub mod prelude {
    pub use crate::{};
}

pub use winit::{
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
};

#[derive(Debug)]
pub enum WindowEvent {
    /// Window Resize event with
    Resized { width: u32, height: u32 },
}

pub fn winit_run(mut app: App, event_loop: EventLoop<()>) {
    println!("Starting Event Loop");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;

        match event {
            winit::event::Event::MainEventsCleared => {
                // TODO do something here?
            }
            winit::event::Event::WindowEvent {
                event,
                window_id,
            } => match event {
                winit::event::WindowEvent::Resized(size) => {
                    // TODO send event to app
                }
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                }
                winit::event::WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {
                    if let Some(window) = windows().get_winit_window(&window_id) {
                        // TODO
                    }
                }
                _ => {},
            }
            _ => {},
        }
    });
}
