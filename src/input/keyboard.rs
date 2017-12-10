use glfw;
use std::convert::From;

#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Key {
    Unknown = 0x00,
    Enter = 0x0D,
    Escape = 0x1B,
    Right = 262,
    Left = 263,
    Down = 264,
    Up = 265,
}

impl From<glfw::Key> for Key {
    fn from(k: glfw::Key) -> Self {
        match k {
            glfw::Key::Enter => Key::Enter,
            glfw::Key::Escape => Key::Escape,
            glfw::Key::Right => Key::Right,
            glfw::Key::Left => Key::Left,
            glfw::Key::Down => Key::Down,
            glfw::Key::Up => Key::Up,
            _ => Key::Unknown,
        }
    }
}

#[test]
fn test_key_input() {
    let key = glfw::Key::Escape;
    assert_eq!(Key::Escape, key.into());
}