pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Rect { x, y, width, height }
    }

    /// Returns the top value of the rect
    pub fn top(&self) -> i32 {
        self.y
    }

    /// Returns the bottom value of the rect
    pub fn bottom(&self) -> i32 {
        self.y + self.height
    }

    /// Returns left value of the rect
    pub fn left(&self) -> i32 {
        self.x
    }

    /// Returns right side value of the rect
    pub fn right(&self) -> i32 {
        self.x + self.width
    }
}
