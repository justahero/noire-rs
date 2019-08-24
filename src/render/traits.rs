/// Used mostly for OpenGL instances that need to be bound before
/// they can be used.
pub trait Bindable {
    /// Binds the instance, should map the associated OpenGL bind function
    fn bind(&mut self) -> &mut Self;
    /// Unbinds the instance, release the resource
    fn unbind(&mut self) -> &mut Self;
    /// Checks if the resource is bound
    fn bound(&self) -> bool;
}

/// Trait to render something
pub trait Drawable {
    fn draw(&mut self);
}
