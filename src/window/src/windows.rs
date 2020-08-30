
use crate::{WindowId, Window};

use std::collections::HashMap;

use winit::window::Window as WinitWindow;
use winit::window::WindowId as WinitWindowId;

pub struct Windows {
    /// Lookup table to find a winit Window by winit internal window id
    pub winit_windows: HashMap<WinitWindowId, WinitWindow>,
    /// Lookup table to find find winit Window by WindowId
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
    pub fn create(&mut self, window: Window, event_loop: &winit::event_loop::EventLoopWindowTarget<()>) -> WinitWindowId {
        let winit_window = {
            winit::window::WindowBuilder::new()
                .build(&event_loop)
                .unwrap()
        };

        // initialize window
        winit_window.set_title(&window.title);
        winit_window.set_inner_size(winit::dpi::PhysicalSize::new(window.width, window.height));
        winit_window.set_resizable(window.resizable);

        let window_id = winit_window.id().clone();

        // store instance of the Window
        self.winit_windows.insert(winit_window.id(), winit_window);
        self.windows.insert(window.id, window);

        window_id
    }

    /// Returns the mutable instance to access winit Window by internal id
    pub fn get_mut_window(&mut self, window_id: &WinitWindowId) -> Option<&mut WinitWindow> {
        self.winit_windows.get_mut(window_id)
    }
}
