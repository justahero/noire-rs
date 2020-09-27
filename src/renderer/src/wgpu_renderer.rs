use std::sync::Arc;
use window::{Window, windows};

/// The main WGPU Renderer that acts as an API layer to WGPU
pub struct WgpuRenderer {
    /// The WGPU instance, used to create Adapters or Surfaces
    pub instance: wgpu::Instance,
    /// The link / connection to the graphics device, useful to create objects
    pub device: Arc<wgpu::Device>,
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
    pub async fn new(window: &Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let windows = windows().lock().unwrap();
        let winit_window = windows.get_winit_window(&window.id).unwrap();

        // setup basic swap chain here for now
        // TODO move to a more appropriate place, especially when resize events occur
        let surface = unsafe { instance.create_surface(winit_window) };

        let width = window.width;
        let height = window.height;

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
            device: Arc::new(device),
            queue,
            surface,
            width,
            height,
        }
    }

    // TODO handle creation of surfaces here?
}
