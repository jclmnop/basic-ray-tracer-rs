use crate::{Camera, Colour, Point, Ray, Vector3D};
use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub struct Intersection(pub Point, pub Colour);

pub trait Shape: Sync {
    /// Calculate where the closes intersection between a ray and the surface of a
    /// shape is, relative to the origin of the ray, if it exists
    fn intersection(&self, ray: &Ray) -> Option<Intersection>;

    /// Calculate the surface normal for a point on the shape, normalised to
    /// a unit vector
    fn surface_normal(&self, point: &Point) -> Vector3D;

    /// Scale the object depending on its distance from camera
    fn scale(&self, ray: &Ray) -> f64;
}

#[derive(Copy, Clone)]
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
            colour,
        }
    }
}

impl Shape for Sphere {
    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        let v = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * (v.dot(&ray.direction));
        let c = v.dot(&v) - (self.radius * self.radius); //* self.scale(ray));
        if let Some(t) = solve_t(a, b, c) {
            Some(Intersection(ray.get_point(t), self.colour))
        } else {
            None
        }
    }

    fn surface_normal(&self, point: &Point) -> Vector3D {
        let mut surface_normal = *point - self.center;
        surface_normal.normalise();

        surface_normal
    }

    fn scale(&self, ray: &Ray) -> f64 {
        let ray_direction = ray.direction;
        let ray_origin = ray.origin;
        let distance = self.center - ray_origin;
        let adjusted_distance = Point::new(
            distance.x * ray_direction.x,
            distance.y * ray_direction.y,
            distance.z * ray_direction.z,
        );
        let mut scale = 250.0 / adjusted_distance.magnitude();
        if scale < 0.0 {
            scale = 0.0;
        }
        scale
    }
}

fn solve_t(a: f64, b: f64, c: f64) -> Option<f64> {
    let discriminant = (b * b) - (4.0 * a * c);
    match discriminant.total_cmp(&0.0) {
        Ordering::Less => None,
        Ordering::Equal => Some((-b) / (2.0 * a)),
        Ordering::Greater => {
            let plus_solution = ((-b) + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
            let minus_solution = ((-b) - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
            Some(plus_solution.max(minus_solution))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vector3D;

    #[test]
    fn no_solution_when_disc_is_negative() {
        assert!(solve_t(2.0, 2.0, 2.0).is_none());
    }

    #[test]
    fn solution_is_correct_when_disc_is_positive() {
        assert_eq!(solve_t(-2.0, 2.0, 1.0), Some(1.3660254037844386));
    }

    #[test]
    fn solution_is_correct_when_disc_is_zero() {
        assert_eq!(solve_t(1.0, 2.0, 1.0), Some(-1.0));
    }

    #[test]
    fn ray_hits_sphere() {
        let c = Point::new(0.0, 0.0, 0.0);
        let r = 100.0;
        let colour = Colour::new(255, 0, 0);
        let sphere = Sphere::new(c, r, colour);
        let ray_origin = Point::new(0.0, 0.0, -200.0);
        let ray_direction = Vector3D::new(0.0, 0.0, 1.0);
        let ray = Ray {
            origin: ray_origin,
            direction: ray_direction,
        };

        let intersection = sphere.intersection(&ray);

        assert!(intersection.is_some());
    }

    #[test]
    fn ray_misses_sphere() {
        let c = Point::new(0.0, 0.0, 0.0);
        let r = 100.0;
        let colour = Colour::new(255, 0, 0);
        let sphere = Sphere::new(c, r, colour);
        let ray_origin = Point::new(0.0, 300.0, -200.0);
        let ray_direction = Vector3D::new(0.0, 0.0, 1.0);
        let ray = Ray {
            origin: ray_origin,
            direction: ray_direction,
        };

        let intersection = sphere.intersection(&ray);

        assert!(intersection.is_none());
    }
}
