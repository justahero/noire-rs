use glfw;

/// a mouse button
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MouseButton {
    /// left mouse button
    Left,
    /// center or middle mouse button
    Middle,
    /// right mouse button
    Right,
    /// unknown mouse button
    Unknown,
}

impl From<glfw::MouseButton> for MouseButton {
    fn from(b: glfw::MouseButton) -> Self {
        match b {
            glfw::MouseButtonLeft => MouseButton::Left,
            glfw::MouseButtonMiddle => MouseButton::Middle,
            glfw::MouseButtonRight => MouseButton::Right,
            _ => MouseButton::Unknown,
        }
    }
}
