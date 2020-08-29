use uuid::Uuid;

#[derive(Copy, Clone, Debug)]
pub struct WindowId(Uuid);

#[derive(Debug)]
pub struct Window {
    pub id: WindowId,
}

struct WindowOptions {
}

impl Default for WindowOptions {
}

impl Window {
    /// Creates a new Window
    pub fn create(id: WindowId, options: WindowOptions) -> Self {
        Self {
            id
        }
    }
}
