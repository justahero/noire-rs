use winit::monitor::{MonitorHandle, VideoMode};

#[derive(Debug, Clone)]
pub enum WindowMode {
    Windowed,
    BorderlessFullscreen,
    Fullscreen,
}

#[derive(Debug)]
/// Description of how a Window instance should be created
pub struct Window {
    /// The title of the Window
    pub title: String,
    /// Width of the Window
    pub width: u32,
    /// Height of the Window
    pub height: u32,
    /// Marks the Window as resizable when true
    pub resizable: bool,
}

impl Window {
    /// Creates a new Window description
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            .. Default::default()
        }
    }
}

impl Default for Window {
    fn default() -> Self {
        Self {
            title: String::from("Hello World"),
            width: 1280,
            height: 720,
            resizable: false,
        }
    }
}

#[derive(Debug)]
/// Description of how a Window instance should be created
pub struct WindowSettings {
    /// The title of the Window
    pub title: String,
    /// Width of the Window
    pub width: u32,
    /// Height of the Window
    pub height: u32,
    /// Marks the Window as resizable when true
    pub resizable: bool,
    /// True when vertical sync is set, limit frame refresh to display to avoid tearing
    pub vsync: bool,
    /// The window mode, fullscreen / windowed
    pub window_mode: WindowMode,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: String::from("Hello World"),
            width: 1280,
            height: 720,
            resizable: false,
            vsync: true,
            window_mode: WindowMode::Windowed,
        }
    }
}

impl WindowSettings {
    /// Creates a new Window description
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            ..Default::default()
        }
    }

    /// Sets the title of the Window
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Sets the dimensions of the Window
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Sets the Window mode
    pub fn with_mode(mut self, mode: WindowMode) -> Self {
        self.window_mode = mode;
        self
    }

    pub fn create_builder(
        &self,
        event_loop: &winit::event_loop::EventLoopWindowTarget<()>,
    ) -> winit::window::WindowBuilder {
        let window_builder = winit::window::WindowBuilder::new();

        match self.window_mode {
            WindowMode::Windowed => window_builder
                .with_inner_size(winit::dpi::PhysicalSize::new(self.width, self.height)),
            WindowMode::Fullscreen => {
                let monitor = event_loop.primary_monitor().unwrap();
                let video_mode = get_video_mode(&monitor).expect("No Video mode found");
                let fullscreen = winit::window::Fullscreen::Exclusive(video_mode);
                window_builder.with_fullscreen(Some(fullscreen))
            }
            WindowMode::BorderlessFullscreen => {
                let monitor = event_loop.primary_monitor();
                let fullscreen = Some(winit::window::Fullscreen::Borderless(monitor));
                window_builder.with_fullscreen(fullscreen)
            }
        }
        .with_title(&self.title)
        .with_resizable(self.resizable)
    }

    pub fn create_window(
        &self,
        event_loop: &winit::event_loop::EventLoopWindowTarget<()>,
    ) -> winit::window::Window {
        self.create_builder(event_loop).build(event_loop).expect("Failed to create window")
    }
}

impl From<&WindowSettings> for wgpu::SwapChainDescriptor {
    fn from(window: &WindowSettings) -> Self {
        let present_mode = match window.vsync {
            true => wgpu::PresentMode::Fifo,
            false => wgpu::PresentMode::Immediate,
        };

        wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: window.width,
            height: window.height,
            present_mode,
        }
    }
}

/// Function to get the current video mode resolution of the current desktop.
/// It enumerates video modes and tries to find the best suitable video mode for the screen resolution.
pub fn get_video_mode(monitor: &MonitorHandle) -> Option<VideoMode> {
    let _size = monitor.size();
    let video_modes = monitor.video_modes().collect::<Vec<_>>();

    println!("VIDEO MODES: {:?}", video_modes);

    todo!("Please implement");
}
