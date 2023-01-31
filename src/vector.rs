use num::{Bounded, Num, NumCast, ToPrimitive};
use std::fmt::{Display, Formatter};

pub trait VectorNum:
    Copy + NumCast + Num + PartialOrd<Self> + Clone + Bounded + Display + ToPrimitive
{
}

impl VectorNum for usize {}
impl VectorNum for u8 {}
// impl VectorNum for u32 {}
// impl VectorNum for u64 {}
impl VectorNum for i32 {}
// impl VectorNum for i64 {}
// impl VectorNum for f32 {}
impl VectorNum for f64 {}

pub type Point = Vector<usize>;
pub type Colour = Vector<u8>;

// TODO: make generic over
//       T: Copy + NumCast + Num + PartialOrd<Self> + Clone + Bounded
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

    /// Magnitude of a Vector
    /// sqrt(x^2 + y^2 + z^2)
    pub fn magnitude(self) -> f64 {
        let squared_vec = self * self;
        let square_sum: T = squared_vec.vec_sum();
        square_sum
            .to_f64()
            .unwrap_or_else(|| panic!("can't represent {square_sum} as f64"))
            .sqrt()
    }

    /// Sum of a Vector
    pub fn vec_sum(&self) -> T {
        self.x + self.y + self.z
    }

    /// Dot product of two Vectors
    pub fn dot(self, other: Self) -> T {
        (self * other).vec_sum()
    }

    pub fn to_f64(self) -> Vector<f64> {
        Vector {
            x: self.x.to_f64().unwrap(),
            y: self.y.to_f64().unwrap(),
            z: self.z.to_f64().unwrap(),
        }
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
}

impl<T: VectorNum> std::ops::Mul<Vector<T>> for Vector<T> {
    type Output = Self;

    /// Cross product of two Vectors of same type
    fn mul(self, rhs: Vector<T>) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
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

impl<T: VectorNum> std::ops::Add<Vector<T>> for Vector<T> {
    type Output = Vector<T>;

    /// Add two Vectors of same type
    fn add(self, rhs: Vector<T>) -> Self::Output {
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
        assert_eq!(v1 * v2, Vector::new(6.0, 20.0, 12.0));
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
}
