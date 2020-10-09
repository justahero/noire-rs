use winit::event_loop::ControlFlow;

use crate::{App, Renderer, Window, WindowHandler, WindowSettings};

#[derive(Debug)]
pub struct EventLoop {
    /// The window event loop
    pub event_loop: winit::event_loop::EventLoop<()>,
}

impl EventLoop {
    pub fn new() -> Self {
        Self {
            event_loop: winit::event_loop::EventLoop::new(),
        }
    }

    /// Runs the window event loop
    pub fn run<T>(self, settings: &WindowSettings, mut app: App)
    where
        T: WindowHandler + Sized + 'static
    {
        println!("Starting Event Loop");
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = winit::event_loop::ControlFlow::Poll;

            match event {
                winit::event::Event::NewEvents(cause) => match cause {
                    winit::event::StartCause::Init => {
                        // event_handler.init(&mut app.resources)
                    }
                    _ => {},
                }
                winit::event::Event::MainEventsCleared => {
                    app.update();
                }
                winit::event::Event::RedrawRequested(_window_id) => {
                }
                winit::event::Event::WindowEvent {
                    event,
                    window_id,
                } => match event {
                    winit::event::WindowEvent::Resized(size) => {
                        // TODO send event to app
                        // let (width, height) = size;
                        let _width = size.width;
                        let _height = size.height;
                        // app.add_event(WindowEvent::Resized{ width, height });
                    }
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit
                    }
                    winit::event::WindowEvent::KeyboardInput { device_id: _, input: _, is_synthetic: _ } => {
                        if let Some(_window) = app.get_window_by_id(&window_id) {
                            // TODO
                        }
                    }
                    _ => {},
                }
                _ => {},
            }
        });
    }
}
