use std::fmt::{Display, Formatter};

// TODO: make generic over
//       T: Copy + NumCast + Num + PartialOrd<Self> + Clone + Bounded
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Magnitude of a Vector
    /// sqrt(x^2 + y^2 + z^2)
    pub fn magnitude(&self) -> f64 {
        (&self.x * &self.x) + (&self.y * &self.y) + (&self.z * &self.z).sqrt()
    }

    /// Normalise a Vector in relation to its magnitude
    pub fn normalise(&mut self) {
        let magnitude = self.magnitude();
        if magnitude != 0.0 {
            self.x /= magnitude;
            self.y /= magnitude;
            self.z /= magnitude;
        }
    }

    /// Sum of a Vector
    pub fn sum(&self) -> f64 {
        self.x + self.y + self.z
    }

    /// Dot product of two Vectors
    pub fn dot(self, other: Self) -> f64 {
        (self * other).sum()
    }
}

/// Cross product of two Vectors
impl std::ops::Mul<Vector> for Vector {
    type Output = Self;

    fn mul(self, rhs: Vector) -> Self::Output {
        Self {x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z}
    }
}

/// Scalar multiplication of a Vector
impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

/// Scalar multiplication of a Vector
impl std::ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Self::Output {x: self * rhs.x, y: self * rhs.y, z: self * rhs.z}
    }
}

/// Add two Vectors
impl std::ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

/// Subtract two Vectors
impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector;

    #[test]
    fn cross_product() {
        let v1 = Vector::new(3.0, 5.0, 2.0);
        let v2 = Vector::new(2.0, 4.0, 6.0);
        assert_eq!(v1 * v2, Vector::new(6.0, 20.0, 12.0));
    }

    #[test]
    fn scalar_mul() {
        let v1 = Vector::new(3.0, 5.0, 2.0);
        assert_eq!(v1 * 2.0, Vector::new(6.0, 10.0, 4.0));
        assert_eq!(2.0 * v1, Vector::new(6.0, 10.0, 4.0));
    }

    #[test]
    fn vector_addition() {
        let v1 = Vector::new(3.0, 5.0, 2.0);
        let v2 = Vector::new(2.0, 4.0, 6.0);
        assert_eq!(v1 + v2, Vector::new(5.0, 9.0, 8.0));
    }

    #[test]
    fn vector_subtraction() {
        let v1 = Vector::new(3.0, 5.0, 2.0);
        let v2 = Vector::new(2.0, 4.0, 6.0);
        assert_eq!(v1 - v2, Vector::new(1.0, 1.0, -4.0));
    }
}