use std::ops::{Add, Sub, AddAssign, MulAssign, Mul};

pub type Point2 = Vector2;

#[derive(Copy, Clone, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

/// Rotates the given vector by angle, returns the rotated Vector2
fn rotate(v: &Vector2, angle: cgmath::Rad<f32>) -> Vector2 {
    let c = angle.0.cos();
    let s = angle.0.sin();
    let x = v.x * c - v.y * s;
    let y = v.x * s + v.y * c;
    Vector2 {
        x, y
    }
}

impl Vector2 {
    /// Construct a new vector
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Normalizes the vector to unit length
    pub fn normalize(&mut self) {
        let l = self.length();
        self.x /= l;
        self.y /= l;
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

impl Mul<cgmath::Rad<f32>> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: cgmath::Rad<f32>) -> Self::Output {
        rotate(&self, rhs)
    }
}

impl Mul<cgmath::Deg<f32>> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: cgmath::Deg<f32>) -> Self::Output {
        rotate(&self, rhs.into())
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
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
