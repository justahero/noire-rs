use std::ops::Sub;

pub type Point3 = Vector3;

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    /// Construct a new Vector3
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Calculate distance to another Vector3
    pub fn distance(self, rhs: Vector3) -> f32 {
        (rhs - self).length()
    }

    /// Calculates the length of the Vector3
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vector3;

    #[test]
    fn vector_returns_length() {
        assert_eq!(Vector3::new(0.0, 3.0, 4.0).length(), 5.0);
    }

    #[test]
    fn vector_calculates_distance() {
        let v1 = Vector3::new(0.0, 1.0, 1.0);
        let v2 = Vector3::new(3.0, 5.0, 1.0);
        assert_eq!(v1.distance(v2), 5.0);
    }
}
