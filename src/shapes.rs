use crate::{Colour, Point, Ray};
use std::cmp::{Ordering};


pub struct Intersection(pub Point, pub Colour);

pub trait Shape {
    fn intersection(&self, ray: &Ray) -> Option<Intersection>;
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub colour: Colour,
}

impl Sphere {
    pub fn new(c: Point, r: f64, colour: Colour) -> Self {
        Sphere {
            center: c,
            radius: r,
            colour
        }
    }
}

impl Shape for Sphere {
    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        let v = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * (v.dot(ray.direction));
        let c = v.dot(v) - (self.radius * self.radius);
        if let Some(t) = solve_t(a, b, c) {
            Some(Intersection(ray.get_point(t), self.colour))
        } else {
            None
        }
    }
}

fn solve_t(a: f64, b: f64, c: f64) -> Option<f64> {
    let discriminant = (b * b) - (4.0 * a * c);
    match discriminant.total_cmp(&0.0) {
        Ordering::Less => None,
        Ordering::Equal => Some((-b) / (2.0 * a)),
        Ordering::Greater => {
            let plus_solution = (-b + (b * b - 4.0 * a * c)) / 2.0 * a;
            let minus_solution = (-b - (b * b - 4.0 * a * c)) / 2.0 * a;
            Some(plus_solution.max(minus_solution))
        }
    }

}
