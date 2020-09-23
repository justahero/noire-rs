use app::prelude::{App, EventHandler};
use window::{Window, WindowMode, Windows, winit_run};
use renderer::{WgpuContext, WgpuRenderer, TextureDescriptor, TextureViewDescriptor};

extern crate noire;
extern crate futures;
extern crate wgpu;

pub struct Example {
    pub window: Window,
}

impl Example {
    pub fn new(window: Window) -> Self {
        Self {
            window,
        }
    }
}

impl EventHandler for Example {
    /// Initializes the Example
    fn init(&mut self) {
    }

    /// Update the example
    fn update(&mut self) {
    }

    /// Renders everything
    fn render(&mut self) {
    }
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
    let event_loop = winit::event_loop::EventLoop::new();
    let mut windows = Windows::default();
    let window = Window::default()
        .with_title("Test")
        .with_mode(WindowMode::Windowed);
    let app = App::build(Example::new(window.clone()));

    let window_id = window.id.clone();

    let winit_id: winit::window::WindowId = windows.create(window, &event_loop);

    let renderer = {
        let winit_window = windows.get_winit_window(&winit_id).unwrap();
        futures::executor::block_on(WgpuRenderer::new(&winit_window))
    };

    let mut context = WgpuContext::new(renderer.device.clone());
    let window = windows.get_window(&window_id).unwrap();
    let _swap_chain = context.create_swapchain(window, &renderer.surface);
    let depth_descriptor = TextureDescriptor::depth(window.width, window.height);
    let depth_texture = context
        .create_depth_texture(&depth_descriptor);
    let mut _depth_texture_view = depth_texture
        .create_view(&TextureViewDescriptor::create_from_texture(&depth_descriptor).into());

    winit_run(app, event_loop);
}
