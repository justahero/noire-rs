extern crate glfw;

pub mod keyboard;
pub mod mouse;
pub mod button;

pub use button::{ButtonState, ButtonArgs};
pub use keyboard::Key;
pub use mouse::MouseButton;

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
    /// more complex button state
    Button(ButtonArgs),
    /// Mouse cursor moved
    Move(Motion),
}

/// Convert from Key to Button
impl From<keyboard::Key> for Button {
    fn from(key: keyboard::Key) -> Self {
        Button::Keyboard(key)
    }
}

#[cfg(test)]
mod tests {
    use super::{Button,Input, keyboard, button::ButtonArgs};

    #[test]
    fn test_keypress_button() {
        let button = keyboard::Key::Escape.into();
        let input = Input::Press(button);
        assert_eq!("Press(Keyboard(Escape))", format!("{:?}", input))
    }

    #[test]
    fn test_general_button_state() {
        use super::button::ButtonState;
        let key = keyboard::Key::Escape.into();
        let button = Button::Keyboard(key);

        let args = ButtonArgs {
            button,
            state: ButtonState::Press,
        };

        let input = Input::Button(args);
        assert_eq!(
            "Button(ButtonArgs { state: Press, button: Keyboard(Escape) })",
            format!("{:?}", input)
        )
    }
}
