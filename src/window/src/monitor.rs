use winit::{monitor::MonitorHandle, event_loop::EventLoopWindowTarget};

/// A Video mode describes a fullscreen resolution
#[derive(Debug)]
pub struct VideoMode {
    /// Width of the resolution
    pub width: u32,
    /// Height of the resolution
    pub height: u32,
    /// Monitor refresh rate in hz
    pub refresh_rate: u16,
    /// The bit depth of the pixel
    pub bit_depth: u16,
}

impl From<winit::monitor::VideoMode> for VideoMode {
    fn from(mode: winit::monitor::VideoMode) -> Self {
        VideoMode {
            width: mode.size().width,
            height: mode.size().height,
            refresh_rate: mode.refresh_rate(),
            bit_depth: mode.bit_depth(),
        }
    }
}

/// Struct to keep information about a physical monitor
#[derive(Debug)]
pub struct Monitor {
    /// The name of the monitor
    pub name: Option<String>,
    /// Physical with of the resolution 
    pub width: u32,
    /// Physical height of the resolution
    pub height: u32,
    /// Scale factor for screen resolution
    pub scale: f64,
    /// The list of fullscreen video modes supported by this Monitor.
    pub video_modes: Vec<VideoMode>,
}

impl From<MonitorHandle> for Monitor {
    fn from(handle: MonitorHandle) -> Self {
        let video_modes = handle.video_modes().map(|m| m.into()).collect();

        Self {
            name: handle.name().clone(),
            width: handle.size().width,
            height: handle.size().height,
            scale: handle.scale_factor(),
            video_modes,
        }
    }
}

impl Monitor {
    /// Due to the way winit enumerates monitors, the event loop is given here to access them
    pub fn available(event_loop: &EventLoopWindowTarget<()>) -> Vec<Monitor> {
        event_loop.available_monitors().map(|h| h.into()).collect()
    }

    /// Returns the primary monitor
    pub fn primary(event_loop: &EventLoopWindowTarget<()>) -> Monitor {
        event_loop.primary_monitor().into()
    }
}
