/// Used mostly for OpenGL instances that need to be bound before
/// they can be used.
pub trait Bindable {
    /// Binds the instance, should map the associated OpenGL bind function
    fn bind(&self) -> &Self;
    /// Unbinds the instance, release the resource
    fn unbind(&self) -> &Self;
    /// Checks if the resource is bound
    fn bound(&self) -> bool;
}

pub trait Drawable {
    fn draw(&self) {}
}
