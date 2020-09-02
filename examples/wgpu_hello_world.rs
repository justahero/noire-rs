use renderer::{Renderer, WindowMode, WindowSettings};
use winit::{event_loop::ControlFlow, event::{WindowEvent, Event}};

extern crate noire;
extern crate futures;
extern crate wgpu;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let settings = WindowSettings::default()
        .with_title("Test")
        .with_mode(WindowMode::Windowed);

    let window = settings.create_window(&event_loop);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Wait;

        // TODO handle Window resize event
        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_window_id) => {
                window.request_redraw();
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput{ ref input, .. } => {
                    if input.virtual_keycode == Some(winit::event::VirtualKeyCode::Escape) && input.state == winit::event::ElementState::Pressed {
                        *control_flow = ControlFlow::Exit
                    }
                }
                _ => ()
            }
            _ => ()
        }
    });
}
