use window::{Window, Windows, WindowMode};
use winit::{event_loop::ControlFlow, event::{WindowEvent, Event, self}};
use renderer::{WgpuContext, WgpuRenderer, TextureDescriptor, TextureViewDescriptor};

extern crate noire;
extern crate futures;
extern crate wgpu;

fn render(
    window: &Window,
    queue: &mut wgpu::Queue,
    context: &mut WgpuContext,
    swap_chain: &mut wgpu::SwapChain,
    depth_texture_view: &mut wgpu::TextureView,
) {
    let swap_texture = swap_chain.get_current_frame().unwrap().output;
    context.begin_pass(window, &swap_texture, &depth_texture_view);
    context.finish(queue);
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let mut windows = Windows::default();
    let window = Window::default()
        .with_title("Test")
        .with_mode(WindowMode::Windowed);
    let window_id = window.id.clone();

    let winit_id: winit::window::WindowId = windows.create(window, &event_loop);

    let mut renderer = {
        let winit_window = windows.get_winit_window(&winit_id).unwrap();
        futures::executor::block_on(WgpuRenderer::new(&winit_window))
    };

    let mut context = WgpuContext::new(renderer.device.clone());
    let window = windows.get_window(&window_id).unwrap();
    let mut swap_chain = context.create_swapchain(window, &renderer.surface);
    let depth_descriptor = TextureDescriptor::depth(window.width, window.height);
    let mut depth_texture = context
        .create_depth_texture(&depth_descriptor);
    let mut depth_texture_view = depth_texture
        .create_view(&TextureViewDescriptor::create_from_texture(&depth_descriptor).into());

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Wait;

        // TODO handle Window resize event
        match event {
            Event::MainEventsCleared => {
                let winit_window = windows.get_winit_window(&winit_id).unwrap();
                winit_window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_window_id) => {
                let window = windows.get_window(&window_id).unwrap();
                render(&window, &mut renderer.queue, &mut context, &mut swap_chain, &mut depth_texture_view);
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
