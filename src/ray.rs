use crate::{Point, Vector3D};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3D,
}

impl Ray {
    pub fn get_point(&self, t: f64) -> Point {
        self.origin + (self.direction * t)
    }
}