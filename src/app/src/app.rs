/// App struct
pub struct App {
    /// The runner
    runner: Box<dyn Fn(App)>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            runner: Box::new(run_once),
        }
    }
}

impl App {
    /// Builds a new App
    pub fn build() -> Self {
        Default::default()
    }

    /// Executes the runner
    pub fn run(mut self) {
        let runner = std::mem::replace(&mut self.runner, Box::new(run_once));
        (runner)(self);
    }

    /// Update the app
    pub fn update(&mut self) {
        println!("App::update");
    }

    /// Adds a new event to the app
    pub fn send_event<T>(&mut self) -> &mut Self
    where
        T: Send + Sync + 'static
    {
        // TODO
        self
    }

    pub fn set_runner(&mut self, runner: impl Fn(App) + 'static) -> &mut Self {
        self.runner = Box::new(runner);
        self
    }
}

fn run_once(mut app: App) {
    app.update();
}
