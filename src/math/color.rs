#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Color {
    pub const WHITE: Self = Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
        alpha: 1.0,
    };

    pub const BLACK: Self = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
        alpha: 1.0,
    };

    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn rgb(red: f32, green: f32, blue: f32) -> Self {
        Color {
            red,
            green,
            blue,
            alpha: 1.0
        }
    }

    /// Returns color as array with RGBA components
    pub fn rgba_array(&self) -> [f32; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }

    /// Returns color as array with RGB components
    pub fn rgb_array(&self) -> [f32; 3] {
        [self.red, self.green, self.blue]
    }
}
