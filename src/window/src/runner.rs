use winit::{event_loop::EventLoop, event::Event, event::WindowEvent, event_loop::ControlFlow};

use crate::windows;

#[derive(Debug)]
pub struct Runner {
}

impl Runner {
    pub fn new() -> Self {
        Self {}
    }

    /// Runs the main loop of the window event loop
    pub fn run(&self, event_loop: EventLoop<()>) {
        run(event_loop)
    }
}

fn run(event_loop: EventLoop<()>) {
    println!("Starting Event Loop");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                println!("Main events cleared");
            }
            Event::WindowEvent {
                event,
                window_id,
            } => match event {
                WindowEvent::Resized(_) => {}
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                }
                WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {
                    if let Some(window) = windows().get_winit_window(&window_id) {
                        // TODO
                    }
                }
                _ => {},
            }
            _ => (),
        }
    });
}
