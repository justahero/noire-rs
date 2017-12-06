use glfw;
use std::convert::From;

#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Key {
    Unknown = 0x00,
    Escape = 0x1B,
}

impl From<glfw::Key> for Key {
    fn from(k: glfw::Key) -> Self {
        match k {
            glfw::Key::Escape => Key::Escape,
            _ => Key::Unknown,
        }
    }
}

#[test]
fn test_worlks() {
    let key = glfw::Key::Escape;
    assert_eq!(Key::Escape, key.into());
}