use std::collections::HashMap;

use winit::window::WindowId as WinitWindowId;

use crate::Window;

pub struct App {
    /// Lookup table to find find Window by WindowId
    pub windows: HashMap<WinitWindowId, Window>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }
}

impl App {
    /// Returns a reference to the winit window by its internal id
    pub fn get_window_by_id(&self, winit_id: &WinitWindowId) -> Option<&Window> {
        self.windows.get(winit_id)
    }

    /// Update the App
    pub fn update(&mut self) {
        // TODO
    }
}
