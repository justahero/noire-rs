use crate::{Renderer, SwapChainDescriptor, WgpuInto};

pub struct Surface {
    /// Reference to the associated window
    window: winit::window::Window,
    /// The Wgpu Surface
    pub surface: wgpu::Surface,
    /// The swap chain
    swap_chain: wgpu::SwapChain,
    /// The output target texture
    output: Option<wgpu::SwapChainFrame>,
}

impl Surface {
    /// Creates a new Surface from the given window
    pub fn new(
        window: winit::window::Window,
        renderer: &Renderer,
    ) -> Self {
        let surface = unsafe { renderer.instance.create_surface(&window) };
        let swap_chain = new_swap_chain(&renderer.device, &surface, &window);

        Self {
            window,
            surface,
            swap_chain,
            output: None,
        }
    }

    /// Returns the associated winit Window
    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }

    /// Returns the target texture to render on
    pub fn texture(&mut self) -> &wgpu::TextureView {
        if self.output.is_none() {
            let frame = self
                .swap_chain
                .get_current_frame()
                .expect("Get current frame");
            self.output = Some(frame);
        };
        &self.output.as_ref().unwrap().output.view
    }

    /// Returns width of the surface
    pub fn width(&self) -> u32 {
        self.window.inner_size().width
    }

    /// Returns height of the surface
    pub fn height(&self) -> u32 {
        self.window.inner_size().height
    }

    /// Returns the Swap Chain descriptor
    pub fn swap_chain_descriptor(&self) -> SwapChainDescriptor {
        (&self.window).into()
    }
}

/// Helper function to create a new swap chain
fn new_swap_chain(
    device: &wgpu::Device,
    surface: &wgpu::Surface,
    window: &winit::window::Window,
) -> wgpu::SwapChain {
    let descriptor = window.wgpu_into();
    device.create_swap_chain(surface, &descriptor)
}
