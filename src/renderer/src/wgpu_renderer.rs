/// The main WGPU Renderer that acts as an API layer to WGPU
pub struct WgpuRenderer {
    /// The WGPU instance, used to create Adapters or Surfaces
    pub instance: wgpu::Instance,
    /// The link / connection to the graphics device, useful to create objects
    pub device: wgpu::Device,
    /// Handle to a command queue on the (graphics) device
    pub queue: wgpu::Queue,
}

pub(crate) async fn get_adapter(instance: &wgpu::Instance) -> Option<wgpu::Adapter> {
    instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
        })
        .await
}

impl WgpuRenderer {
    pub async fn new() -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let adapter = get_adapter(&instance).await.expect("Unable to find GPU!");

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

        // setup basic

        Self {
            instance,
            device,
            queue,
        }
    }
}
