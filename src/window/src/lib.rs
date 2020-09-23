pub mod window;
pub mod windows;

use app::prelude::{App, EventHandler};
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
    /// Update
    Update,
    /// Window is closed
    CloseWindow { window_id: WindowId },
}

pub fn winit_run<E>(mut app: App<E>, event_loop: EventLoop<()>)
where
    E: EventHandler
{
    println!("Starting Event Loop");
    app.event_handler.init();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;

        match event {
            winit::event::Event::MainEventsCleared => {
                app.update();
            }
            winit::event::Event::Suspended => {

            }
            winit::event::Event::Resumed => {

            }
            winit::event::Event::WindowEvent {
                event,
                window_id,
            } => match event {
                winit::event::WindowEvent::Resized(size) => {
                    // let (width, height) = size;
                    let width = size.width;
                    let height = size.height;
                    app.add_event(WindowEvent::Resized{ width, height });
                }
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                }
                winit::event::WindowEvent::KeyboardInput { device_id: _, input: _, is_synthetic: _ } => {
                    if let Some(_window) = windows().lock().unwrap().get_winit_window_by_id(&window_id) {
                        // TODO
                    }
                }
                _ => {},
            }
            _ => {},
        }
    });
}
