use num::{Bounded, Num, NumCast, ToPrimitive};
use std::fmt::{Display, Formatter};

pub trait VectorNum:
    Copy
    + NumCast
    + Num
    + PartialOrd<Self>
    + Clone
    + Bounded
    + Display
    + ToPrimitive
{
}

impl VectorNum for u8 {}
impl VectorNum for f64 {}

/// Represents a point in 3D space
pub type Point = Vector<f64>;
/// Represents RGB<u8> value for a pixel
pub type PixelColour = Vector<u8>;
/// Usually represents a direction in 3D space, can be a regular vector or can
/// be a unit vector.
pub type Vector3D = Vector<f64>;
/// Represents the colour of a light source/ray.
/// Range from 0.0 to 1.0
pub type LightColour = Vector<f64>;

#[derive(Clone, Copy)]
pub enum ColourChannel {
    Red,
    Green,
    Blue,
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vector<T: VectorNum> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: VectorNum> Vector<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn from_array(a: [T; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }

    pub fn to_array(&self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    /// Magnitude of a Vector
    /// sqrt(x^2 + y^2 + z^2)
    pub fn magnitude(self) -> f64 {
        let squared_vec = self.square();
        let square_sum: T = squared_vec.vec_sum();
        square_sum
            .to_f64()
            .unwrap_or_else(|| panic!("can't represent {square_sum} as f64"))
            .sqrt()
    }

    /// Square the values inside a vector
    pub fn square(&self) -> Vector<T> {
        Vector::new(self.x * self.x, self.y * self.y, self.z * self.z)
    }

    /// Sum of a Vector
    pub fn vec_sum(&self) -> T {
        self.x + self.y + self.z
    }

    /// Dot product of two Vectors
    pub fn dot(self, other: &Self) -> T {
        Vector::new(self.x * other.x, self.y * other.y, self.z * other.z)
            .vec_sum()
    }

    /// Multiply two vectors of same type by their values
    /// Note: This is not the cross product
    pub fn mul(&self, rhs: &Self) -> Vector<T> {
        Vector::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.y)
    }

    pub fn colour(&self, colour_channel: &ColourChannel) -> T {
        match colour_channel {
            ColourChannel::Red => self.x,
            ColourChannel::Green => self.y,
            ColourChannel::Blue => self.z,
        }
    }

    pub fn to_f64(self) -> Vector<f64> {
        Vector {
            x: self.x.to_f64().unwrap(),
            y: self.y.to_f64().unwrap(),
            z: self.z.to_f64().unwrap(),
        }
    }
}

//TODO: u8 -> f64 for calculations more efficient?
//      or f64 -> u8 for pixels more efficient?
impl Vector<u8> {
    pub fn from_light_colour(light_colour: &LightColour) -> Self {
        PixelColour::from_array([
            (light_colour.x * 255.0) as u8,
            (light_colour.y * 255.0) as u8,
            (light_colour.z * 255.0) as u8,
        ])
    }

    pub fn to_light_colour(&self) -> LightColour {
        LightColour::from_array([
            self.x as f64 / 255.0,
            self.y as f64 / 255.0,
            self.z as f64 / 255.0,
        ])
    }
}

impl Vector<f64> {
    /// Normalise a `Vector<f64>` in relation to its magnitude
    pub fn normalise(&mut self) {
        let magnitude = self.magnitude();
        if magnitude != 0.0 {
            self.x /= magnitude;
            self.y /= magnitude;
            self.z /= magnitude;
        }
    }

    /// Calculate the cosine of the degree between two vectors
    pub fn cosine_angle(&self, other: &Vector<f64>) -> f64 {
        // 9.0 is the product of both vector lengths
        self.dot(other) / 9.0
    }

    pub fn invert(&self) -> Vector<f64> {
        Self::new(
            if self.x != 0.0 { 1.0 / self.x } else { 0.0 },
            if self.y != 0.0 { 1.0 / self.y } else { 0.0 },
            if self.z != 0.0 { 1.0 / self.z } else { 0.0 },
        )
    }
}

impl<T: VectorNum> std::ops::Mul<Vector<T>> for Vector<T> {
    type Output = Self;

    /// Cross product of two Vectors of same type
    fn mul(self, rhs: Vector<T>) -> Self::Output {
        Self {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }
}

impl<T: VectorNum> std::ops::Mul<T> for Vector<T> {
    type Output = Vector<T>;

    /// Scalar multiplication of a Vector
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: VectorNum> std::ops::Div<Vector<T>> for Vector<T> {
    type Output = Self;

    /// Value division of two vectors
    fn div(self, rhs: Vector<T>) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T: VectorNum> std::ops::Div<T> for Vector<T> {
    type Output = Self;

    /// Scalar division of a vector
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::Add<Vector<u8>> for Vector<u8> {
    type Output = Vector<u8>;

    /// Add two Vector<u8>
    /// Saturates at T.max_value()
    fn add(self, rhs: Vector<u8>) -> Self::Output {
        Self {
            x: self.x.saturating_add(rhs.x),
            y: self.y.saturating_add(rhs.y),
            z: self.z.saturating_add(rhs.z),
        }
    }
}

impl std::ops::Add<Vector<f64>> for Vector<f64> {
    type Output = Vector<f64>;

    /// Add two Vector<f64>
    fn add(self, rhs: Vector<f64>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: VectorNum> std::ops::Sub<Vector<T>> for Vector<T> {
    type Output = Vector<T>;

    /// Subtract two Vectors of same type
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: VectorNum> std::ops::Sub<T> for Vector<T> {
    type Output = Vector<T>;

    /// Subtract a scalar from a vector
    fn sub(self, rhs: T) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl<T: VectorNum> Display for Vector<T> {
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
        assert_eq!(v1 * v2, Vector::new(22.0, -14.0, 2.0));
    }

    #[test]
    fn scalar_mul() {
        let v1 = Vector::new(3.0, 5.0, 2.0);
        assert_eq!(v1 * 2.0, Vector::new(6.0, 10.0, 4.0));
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

    #[test]
    fn vector_not_consumed_after_magnitude() {
        let v = Vector::new(1, 3, 2);
        let _ = v.magnitude();
        let _ = v * v;
    }

    #[test]
    fn vector_to_f64_works() {
        let v1 = Vector::new(1, 3, 4);
        let v2 = v1.to_f64();
        assert_eq!(v2, Vector::new(1.0, 3.0, 4.0));
    }

    #[test]
    fn magnitude_works() {
        let v = Vector::new(0.0, 0.0, 3.0);
        assert_eq!(v.magnitude(), 3.0);
    }

    #[test]
    fn dot_product() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(1.0, 5.0, 7.0);
        assert_eq!(v1.dot(&v2), 32.0);
    }
}
