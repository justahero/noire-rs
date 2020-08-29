#[derive(Debug, Clone)]
pub enum WindowMode {
    Windowed,
    BorderlessFullscreen,
    Fullscreen,
}

/// Specifies a renderable Window
#[derive(Debug)]
/// Description of how a Window instance should be created
pub struct Window {
    /// The title of the Window
    pub title: String,
    /// Width of the Window
    pub width: u32,
    /// Height of the Window
    pub height: u32,
    /// Marks the Window as resizable when true
    pub resizable: bool,
}

impl Window {
    /// Creates a new Window description
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            .. Default::default()
        }
    }
}

impl Default for Window {
    fn default() -> Self {
        Self {
            title: String::from("Hello World"),
            width: 1280,
            height: 720,
            resizable: false,
        }
    }
}
