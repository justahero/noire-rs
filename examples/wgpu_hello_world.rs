use window::{Window, Windows, WindowMode};
use renderer::WgpuRenderer;
use winit::{event_loop::ControlFlow, event::{WindowEvent, Event, self}};

extern crate noire;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let mut windows = Windows::default();
    let window = Window::default()
        .with_title("Test")
        .with_mode(WindowMode::BorderlessFullscreen);
    let _id = windows.create(window, &event_loop);
    // let winit_window = windows.get_mut_window(&id).unwrap();

    // let renderer = WgpuRenderer::new();

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
