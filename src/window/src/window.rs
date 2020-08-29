use uuid::Uuid;

#[derive(Copy, Clone, Debug)]
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
pub struct Window {
    /// The id reference to the window
    pub id: WindowId,
    /// The title of the Window
    pub title: String,
}

/// Default options create a new window
pub struct WindowOptions {
    pub title: String,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            title: String::from("Hello World"),
        }
    }
}

impl Window {
    /// Creates a new Window
    pub fn create(id: WindowId, options: WindowOptions) -> Self {
        Self {
            id,
            title: options.title,
        }
    }
}
