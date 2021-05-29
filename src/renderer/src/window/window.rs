use winit::monitor::{MonitorHandle, VideoMode};

use crate::{Renderer, Surface, Texture};

#[derive(Debug, Clone)]
pub enum WindowMode {
    Windowed,
    BorderlessFullscreen,
    Fullscreen,
}

/// Specifies a renderable Window
#[derive(Debug)]
pub struct Window {
    /// The render surface to render into
    pub surface: Surface,
    /// The depth buffer texture
    pub depth_buffer: Texture,
}

impl Window {
    /// Creates a new renderable window
    pub fn new(
        settings: &WindowSettings,
        event_loop: &winit::event_loop::EventLoopWindowTarget<()>,
        renderer: &Renderer,
    ) -> Self {
        let window_builder = settings.create_builder(&event_loop);
        let window = window_builder.build(&event_loop).unwrap();
        let surface = renderer.create_surface(window);
        let depth_buffer = renderer.create_depth_texture(surface.width(), surface.height());

        Self {
            surface,
            depth_buffer,
        }
    }

    /// Returns the aspect ratio
    pub fn aspect(&self) -> f32 {
        self.surface.aspect()
    }

    /// Returns width of the window
    pub fn width(&self) -> u32 {
        self.surface.width()
    }

    /// Returns height of the window
    pub fn height(&self) -> u32 {
        self.surface.height()
    }

    /// Returns the winit window instance
    pub fn winit_window(&self) -> &winit::window::Window {
        &self.surface.window()
    }

    /// "Swaps" the buffer, by removing the surface output its content is rendered
    pub fn swap_buffers(&mut self) {
        &self.surface.drop_swap_chain();
    }
}

#[derive(Debug, Clone)]
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
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
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
