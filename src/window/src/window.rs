use uuid::Uuid;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WindowId(Uuid);

impl WindowId {
    /// Returns the primary Window Id
    pub fn primary() -> Self {
        WindowId(Uuid::from_u128(0))
    }
}

impl Default for WindowId {
    fn default() -> Self {
        WindowId::primary()
    }
}

#[derive(Debug)]
pub enum WindowMode {
    Windowed,
    BorderlessFullscreen,
    Fullscreen,
}

#[derive(Debug)]
/// Description of how a Window instance should be created
/// The Window implements the Builder pattern, specific attributes can be
/// set independently
pub struct Window {
    /// The id reference to the window
    pub id: WindowId,
    /// The title of the Window
    pub title: String,
    /// Width of the Window
    pub width: u32,
    /// Height of the Window
    pub height: u32,
    /// Marks the Window as resizable when true
    pub resizable: bool,
    /// True when vertical sync is set, limit frame refresh to display to avoid tearing
    pub vsync: bool,
    /// The window mode, fullscreen / windowed
    pub window_mode: WindowMode,
}

impl Window {
    /// Creates a new Window description
    pub fn new(id: WindowId, title: &str, width: u32, height: u32) -> Self {
        Self {
            id,
            title: title.to_string(),
            width,
            height,
            .. Default::default()
        }
    }

    /// Sets the title of the Window
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Sets the dimensions of the Window
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Sets the Window mode
    pub fn with_mode(mut self, mode: WindowMode) -> Self {
        self.window_mode = mode;
        self
    }
}

impl Default for Window {
    fn default() -> Self {
        Self {
            id: WindowId::primary(),
            title: String::from("Hello World"),
            width: 1280,
            height: 720,
            resizable: false,
            vsync: true,
            window_mode: WindowMode::Windowed,
        }
    }
}
