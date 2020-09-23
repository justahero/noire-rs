use crate::prelude::EventHandler;

/// App struct
pub struct App<E: EventHandler + 'static> {
    /// The runner
    runner: Box<dyn Fn(App<E>)>,
    /// Keep the example
    pub event_handler: E,
}

impl<E> App<E>
where
    E: EventHandler + 'static
{
    /// Builds a new App
    pub fn build(instance: E) -> Self {
        Self {
            runner: Box::new(run_once),
            event_handler: instance,
        }
    }

    /// Executes the runner
    pub fn run(mut self) {
        let runner = std::mem::replace(&mut self.runner, Box::new(run_once));
        (runner)(self);
    }

    /// Update the app
    pub fn update(&mut self) {
        self.event_handler.update();
    }

    /// Adds a new event to the app
    pub fn add_event<T>(&mut self, _event: T) -> &mut Self {
        self
    }

    pub fn set_runner(&mut self, runner: impl Fn(App<E>) + 'static) -> &mut Self {
        self.runner = Box::new(runner);
        self
    }
}

fn run_once<E>(mut app: App<E>)
where
    E: EventHandler
{
    app.update();
}
