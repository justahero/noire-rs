use glfw;

/// a mouse button
#[derive(Copy, Clone, Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
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
