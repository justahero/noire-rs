use crate::{App, Window};

pub struct AppBuilder {
    /// The app that is run
    pub app: App,
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self {
            app: App::default(),
        }
    }
}

impl AppBuilder {
    /// Sets up to create a new window later
    pub fn create_window(&mut self, window: Window) -> &mut Self {
        self.app.windows.push(window);
        self
    }

    /// Puts the underlying App into run event loop
    pub fn run(&mut self) {
        // TODO maybe create event loop here, create windows here, setup resources and assign to app?

        let app = std::mem::take(&mut self.app);
        app.run();
    }
}
