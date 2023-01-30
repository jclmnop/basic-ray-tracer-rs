use std::fmt::{Display, Formatter};

#[derive(Default, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub fn normalise(&mut self) {
        let magnitude = self.magnitude();
        if magnitude != 0.0 {
            self.x /= magnitude;
            self.y /= magnitude;
            self.z /= magnitude;
        }
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn sub(&self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn add(&self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn mul(&self, coefficient: f64) -> Self {
        Self {
            x: self.x * coefficient,
            y: self.y * coefficient,
            z: self.z * coefficient,
        }
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}

/*
public class Vector {
  double x, y, z;
  public Vector() {}
  public Vector(double i, double j, double k) {
    x = i;
    y = j;
    z = k;
  }
  public double magnitude() {
    return Math.sqrt(x * x + y * y + z * z);
  }
  public void normalise() {
    double mag = magnitude();
    if (mag != 0) {
      x /= mag;
      y /= mag;
      z /= mag;
    }
  }
  public double dot(Vector a) {
    return x * a.x + y * a.y + z * a.z;
  }
  public Vector sub(Vector a) {
    return new Vector(x - a.x, y - a.y, z - a.z);
  }
  public Vector add(Vector a) {
    return new Vector(x + a.x, y + a.y, z + a.z);
  }
  public Vector mul(double d) {
    return new Vector(d * x, d * y, d * z);
  }
  public void print() {
    System.out.println("x=" + x + ", y=" + y + ", z=" + z);
  }
}
 */
