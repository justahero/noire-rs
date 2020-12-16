use winit::event_loop::ControlFlow;

use crate::{App, Renderer, Window, WindowHandler, WindowSettings};

#[derive(Debug)]
pub struct EventLoop {
    /// The window event loop
    pub event_loop: winit::event_loop::EventLoop<()>,
}

impl<'a> EventLoop {
    pub fn new() -> Self {
        Self {
            event_loop: winit::event_loop::EventLoop::new(),
        }
    }

    /// Runs the window event loop
    pub fn run<T>(self, settings: &WindowSettings, app: App)
    where
        T: WindowHandler + Sized + 'static
    {
        let mut renderer = futures::executor::block_on(Renderer::new());
        let mut window = Window::new(settings, &self.event_loop, &renderer);
        let mut window_handler = T::load(&window, &app.resources, &mut renderer);

        // Start main event loop
        self.event_loop.run(move |event, _event_loop, control_flow| {
            *control_flow = winit::event_loop::ControlFlow::Poll;

            match event {
                winit::event::Event::NewEvents(cause) => match cause {
                    winit::event::StartCause::Init => {
                    }
                    _ => {},
                }
                winit::event::Event::MainEventsCleared => {
                    window_handler.update(&app.resources);
                    window.winit_window().request_redraw();
                }
                winit::event::Event::RedrawRequested(window_id) => {
                    if window.winit_window().id() == window_id {
                        window_handler.render(&mut window, &mut renderer);
                        window.swap_buffers();
                        window.winit_window().request_redraw();
                    }
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
