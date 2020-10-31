use resources::Resources;

use crate::{App, Renderer, Window, WindowSettings, EventLoop};

/// Represents a window to render into, that also updates data and renders
///
pub trait WindowHandler {
    /// Loads the window
    ///
    /// Use this function to load all assets
    fn load(window: &Window, _resources: &Resources, _renderer: &mut Renderer) -> Self where Self: Sized;

    /// Resizes the window
    fn resize(&mut self, _width: u32, _height: u32) {}

    /// Method to update all entities in the window.
    fn update(&mut self, _resources: &Resources);

    /// Renders data to the frame buffer
    fn render(&mut self, _window: &mut Window, app: &mut Renderer);

    /// Runs the main event loop in context of the window
    fn run(settings: WindowSettings)
    where
        Self: 'static + Sized,
    {
        let event_loop = EventLoop::new();
        let app = App::build();

        event_loop.run::<Self>(&settings, app);
    }
}