use winit::window::Window as WinitWindow;

/// The main WGPU Renderer that acts as an API layer to WGPU
pub struct WgpuRenderer {
    /// The WGPU instance, used to create Adapters or Surfaces
    pub instance: wgpu::Instance,
    /// The link / connection to the graphics device, useful to create objects
    pub device: wgpu::Device,
    /// Handle to a command queue on the (graphics) device
    pub queue: wgpu::Queue,
    /// Default surface
    pub surface: wgpu::Surface,
    /// The width of the surface
    pub width: u32,
    /// The height of the surface
    pub height: u32,
}

impl WgpuRenderer {
    pub async fn new(window: &WinitWindow) -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        // setup basic swap chain here for now
        // TODO move to a more appropriate place, especially when resize events occur
        let surface = unsafe { instance.create_surface(window) };

        let width = window.inner_size().width;
        let height = window.inner_size().height;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Unable to find GPU!");


        // TODO maybe disable it here
        let trace_path = Some(std::path::Path::new("wgpu_trace"));

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                trace_path
            )
            .await
            .expect("Failed to request device.");

        Self {
            instance,
            device,
            queue,
            surface,
            width,
            height,
        }
    }

    /// Creates a new swap chain
    /// TODO move from here, it does not seem a good place
    pub fn create_swapchain(&self, surface: &wgpu::Surface) -> wgpu::SwapChain {
        let descriptor = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8Unorm,
            width: self.width,
            height: self.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        self.device.create_swap_chain(surface, &descriptor)
    }
}
