/// Basic struct to hold 4 components color
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    /// The red channel
    pub r: f32,
    /// The green channel
    pub g: f32,
    /// The blue channel
    pub b: f32,
    /// The alpha channel
    pub a: f32,
}

impl Color {
    pub const WHITE: Self = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Self = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };

    #[inline(always)]
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color {
            r,
            g,
            b,
            a,
        }
    }

    #[inline(always)]
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Color {
            r,
            g,
            b,
            a: 1.0
        }
    }

    /// Returns color as array with RGBA components
    #[inline(always)]
    pub fn rgba_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Returns the rgb values as vec
    #[inline(always)]
    pub fn rgb_vec(&self) -> Vec<f32> {
        vec![self.r, self.g, self.b]
    }

    /// Returns color as array with RGB components
    #[inline(always)]
    pub fn rgb_array(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
}
