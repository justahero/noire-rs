pub struct App {
    /// The runner
    runner: Box<dyn Fn(App)>,
}

impl App {
    pub fn add_event<T>(&mut self) -> &mut Self
    where
        T: Send + Sync + 'static
    {
        self
    }
}
