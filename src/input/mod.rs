pub mod mouse;

use input::mouse::MouseButton;

#[derive(Copy, Clone, Debug)]
pub enum Motion {
    /// x and y in window coordinates
    MousePos(f64, f64),
    /// x and y in relative coordinates
    MouseRelative(f64, f64),
}

#[derive(Copy, Clone, Debug)]
pub enum Button {
    Mouse(MouseButton),
}

#[derive(Copy, Clone, Debug)]
pub enum Input {
    Button(Button),
    Move(Motion),
}
