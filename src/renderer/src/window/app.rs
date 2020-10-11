use std::collections::HashMap;

use resources::Resources;

use winit::window::{Window, WindowId as WinitWindowId};

pub struct App {
    /// Lookup table to find find Window by WindowId
    pub windows: HashMap<WinitWindowId, Window>,
    /// The list of resources
    pub resources: Resources,
}

impl Default for App {
    fn default() -> Self {
        Self {
            windows: HashMap::new(),
            resources: Resources::new(),
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

    /// Builds a new App
    pub fn build() -> Self {
        App::default()
    }

    /// Adds a new resource
    pub fn add_resource<T>(&mut self, value: T) -> &mut Self
    where
        T: Send + Sync + 'static,
    {
        self.resources.insert::<T>(value);
        self
    }
}
