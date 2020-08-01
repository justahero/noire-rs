use std::ops::Sub;

#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    /// Construct a new vector
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Calculate distance to another Vector
    pub fn distance(self, rhs: Vector2) -> f32 {
        (rhs - self).length()
    }

    /// Calculates the length of the Vector
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vector2;

    #[test]
    fn it_returns_length() {
        assert_eq!(Vector2::new(1.0, 0.0).length(), 1.0);
        assert_eq!(Vector2::new(0.0, 5.0).length(), 5.0);
        assert_eq!(Vector2::new(3.0, 4.0).length(), 5.0);
    }

    #[test]
    fn it_calculates_distance() {
        let v1 = Vector2::new(1.0, 1.0);
        let v2 = Vector2::new(4.0, 5.0);
        assert_eq!(v1.distance(v2), 5.0);
    }
}
