use window::{Window, Windows, WindowMode};
use renderer::WgpuRenderer;
use winit::{event_loop::ControlFlow, event::{WindowEvent, Event, self}};

extern crate noire;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let mut windows = Windows::default();
    let window = Window::default()
        .with_title("Test")
        .with_mode(WindowMode::Windowed);
        
    let id = windows.create(window, &event_loop);

    // TODO a basic Wgpu based renderer is a lot of effort to set up.

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Wait;

        match event {
            Event::MainEventsCleared => {
                let winit_window = windows.get_window(&id).unwrap();
                winit_window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_window_id) => {
                // Takes a lot of effort to render something here
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput{ ref input, .. } => {
                    if input.virtual_keycode == Some(event::VirtualKeyCode::Escape) && input.state == event::ElementState::Pressed {
                        *control_flow = ControlFlow::Exit
                    }
                }
                _ => ()
            }
            _ => ()
        }
    });
}
