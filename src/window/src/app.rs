use crate::{AppBuilder, Window, winit_run};

pub struct App {
    /// The runner to run the application with, kind of the main loop
    pub runner: Box<dyn Fn(App)>,
    /// The list of associated windows
    pub windows: Vec<Window>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            runner: Box::new(winit_run),
            windows: Vec::new(),
        }
    }
}

impl App {
    /// Create a new App to use with builder pattern
    pub fn build() -> AppBuilder {
        AppBuilder::default()
    }

    /// Sets the runner for this App
    pub fn set_runner(&mut self, runner: impl Fn(App) + 'static) -> &mut Self {
        self.runner = Box::new(runner);
        self
    }

    /// Executes the runner until it's over, takes ownership of App
    pub fn run(mut self) {
        let runner = std::mem::replace(&mut self.runner, Box::new(winit_run));
        (runner)(self);
    }

    /// Update the App
    pub fn update(&mut self) {
        // TODO
    }
}
