use window::{App, Window};
use renderer::{WgpuContext};

extern crate noire;
extern crate futures;
extern crate wgpu;

pub struct Example {
}

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
    /*
    let event_loop = winit::event_loop::EventLoop::new();
    let window = Window::default()
        .with_title("Test")
        .with_mode(WindowMode::Windowed);
    let _ = windows().lock().unwrap().create(window.clone(), &event_loop);

    let app = App::build(Example::new(window.clone()));
    let renderer = futures::executor::block_on(WgpuRenderer::new(&window));

    let mut context = WgpuContext::new(renderer.device.clone());
    let _swap_chain = context.create_swapchain(&window, &renderer.surface);
    let depth_descriptor = TextureDescriptor::depth(window.width, window.height);
    let depth_texture = context
        .create_depth_texture(&depth_descriptor);
    let mut _depth_texture_view = depth_texture
        .create_view(&TextureViewDescriptor::create_from_texture(&depth_descriptor).into());

    winit_run(app, event_loop);
    */

    App::build()
        .create_window(Window::default())
        .run();
}
