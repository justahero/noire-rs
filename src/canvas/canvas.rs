use math::{Color, Rect};

pub struct Canvas2D {
    /// color to render the next primitive with
    draw_color: Color,
}

impl Canvas2D {
    /// Create a new instance of the canvas
    pub fn new() -> Self {
        Canvas2D {
            draw_color: Color::BLACK,
        }
    }

    /// Clears the canvas, sets it to given colors
    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) -> &Self {
        self
    }

    pub fn set_color(&mut self, color: Color) -> &Self {
        self.draw_color = color;
        self
    }

    /// Draws a single point
    pub fn draw_point(&self) -> &Self {
        self
    }

    /// Draw a rect
    pub fn draw_rect(&self, rect: &Rect) -> &Self {
        self
    }

    /// Renders the content of the canvas.
    pub fn render(&self) {
        // TODO
    }
}
