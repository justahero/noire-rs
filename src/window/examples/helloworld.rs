extern crate window;

use window::*;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let mut windows = Windows::default();
    let id = windows.create(Window::default(), &event_loop);
    let winit_window = windows.get_mut_window(&id).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(window_id) => {
                windows.get_mut_window(&window_id).unwrap().request_redraw();
            }
            _ => ()
        }
    });
}
