/// Trait to handle events, it's the primary interface for most relevant events.
///
pub trait EventHandler {
    /// This is called once initially
    fn init(&mut self) {}

    /// Update the appliaction once every frame, used for updates
    fn update(&mut self) {}

    /// Call to render everything
    fn render(&mut self) {}
}
