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
/// Description of how a Window instance should be created
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
}

impl Default for Window {
    fn default() -> Self {
        Self {
            id: WindowId::primary(),
            title: String::from("Hello World"),
            width: 1280,
            height: 720,
            resizable: false,
        }
    }
}
