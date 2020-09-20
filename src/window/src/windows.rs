use std::{collections::HashMap, sync::Arc, sync::Mutex};
use winit::monitor::{MonitorHandle, VideoMode};

use once_cell::sync::OnceCell;
use winit::window::Window as WinitWindow;
use winit::window::WindowId as WinitWindowId;

use crate::{Window, WindowId, WindowMode};

/// This function creates the single instance of Windows
pub(crate) fn windows() -> &'static Arc<Mutex<Windows>> {
    static WINDOWS: OnceCell<Arc<Mutex<Windows>>> = OnceCell::new();
    WINDOWS.get_or_init(|| Arc::new(Mutex::new(Windows::default())))
}

pub struct Windows {
    /// Lookup table to find a winit Window by winit internal window id
    pub winit_windows: HashMap<WinitWindowId, WinitWindow>,
    /// Lookup table to find find Window by WindowId
    pub windows: HashMap<WindowId, Window>,
}

impl Default for Windows {
    fn default() -> Self {
        Self {
            winit_windows: HashMap::new(),
            windows: HashMap::new(),
        }
    }
}

impl Windows {
    pub fn create(
        &mut self,
        window: Window,
        event_loop: &winit::event_loop::EventLoopWindowTarget<()>,
    ) -> WinitWindowId {
        let mut window_builder = winit::window::WindowBuilder::new();

        window_builder = match window.window_mode {
            WindowMode::Windowed => window_builder
                .with_inner_size(winit::dpi::PhysicalSize::new(window.width, window.height)),
            WindowMode::Fullscreen => {
                let monitor = event_loop.primary_monitor();
                let video_mode = get_video_mode(&monitor).expect("No Video mode found");
                let fullscreen = winit::window::Fullscreen::Exclusive(video_mode);
                window_builder.with_fullscreen(Some(fullscreen))
            }
            WindowMode::BorderlessFullscreen => {
                let monitor = event_loop.primary_monitor();
                let fullscreen = Some(winit::window::Fullscreen::Borderless(monitor));
                window_builder.with_fullscreen(fullscreen)
            }
        };

        let winit_window = window_builder.build(&event_loop).unwrap();

        // initialize window
        winit_window.set_title(&window.title);
        winit_window.set_resizable(window.resizable);

        let window_id = winit_window.id().clone();

        // store instance of the Window
        self.winit_windows.insert(winit_window.id(), winit_window);
        self.windows.insert(window.id, window);

        window_id
    }

    /// Returns a reference to the winit Window by internal id
    pub fn get_winit_window(&self, window_id: &WinitWindowId) -> Option<&WinitWindow> {
        self.winit_windows.get(window_id)
    }

    /// Returns a reference to the Window description
    pub fn get_window(&self, window_id: &WindowId) -> Option<&Window> {
        self.windows.get(window_id)
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
