extern crate glfw;

pub mod keyboard;
pub mod mouse;
pub mod button;

pub use input::button::{ButtonState, ButtonArgs};
pub use input::keyboard::Key;
pub use input::mouse::MouseButton;

/// Enum to support different button types
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Button {
    /// mouse button press
    Mouse(mouse::MouseButton),
    /// keyboard key
    Keyboard(keyboard::Key),
}

/// Enum to capture motion of input devices
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Motion {
    /// x and y in window coordinates
    MousePos(f64, f64),
    /// x and y in relative coordinates
    MouseRelative(f64, f64),
}

/// Enum to support different input types
#[derive(Copy, Clone, Debug)]
pub enum Input {
    /// Button press input
    Press(Button),
    /// Button pressed state
    Pressed(Button),
    /// Button release input
    Release(Button),
    /// Mouse cursor moved
    Move(Motion),
}

/// Convert from Key to Button
impl From<keyboard::Key> for Button {
    fn from(key: keyboard::Key) -> Self {
        Button::Keyboard(key)
    }
}

#[test]
fn button_press() {
    let button = keyboard::Key::Escape.into();
    let input = Input::Press(button);
    assert_eq!("Press(Keyboard(Escape))", format!("{:?}", input))
}

#[test]
fn button_press() {
    use input::button::ButtonState;
    let args = ButtonArgs {
        button: Button::Keyboard(keyboard::Key::Escape.into()),
        state: ButtonState::Press,
    };
    let input = Input::Button(args);
    assert_eq!(
        "Button(ButtonArgs { state: Press, button: Keyboard(Escape) })",
        format!("{:?}", input)
    )
}
