mod camera;
mod ray;
mod render;
mod shapes;
mod vector;

pub use camera::*;
pub use ray::*;
pub use render::*;
pub use vector::*;

// Image parameters TODO: ImageParam struct
pub const IMG_HEIGHT: u32 = 640;
pub const IMG_WIDTH: u32 = 640;

// Test scale

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::{Shape, Sphere};
    use image::RgbaImage;
    use std::path::Path;

    const TEST_PATH: &str = "./test.png";
    const SCALE: f64 = 0.7;

    // Test spheres
    const C: Point = Point {
        x: 0.0,
        y: 0.0,
        z: 100.0,
    };
    const R: f64 = 100.0;
    const COLOUR: Colour = Colour { x: 200, y: 0, z: 0 };
    const TEST_SPHERE: Sphere = Sphere {
        center: C,
        radius: R,
        colour: COLOUR,
    };

    const C2: Point = Point {
        x: 70.0,
        y: 20.0,
        z: -200.0,
    };
    const R2: f64 = 50.0;
    const COLOUR2: Colour = Colour { x: 0, y: 0, z: 200 };
    const TEST_SPHERE2: Sphere = Sphere {
        center: C2,
        radius: R2,
        colour: COLOUR2,
    };

    const C3: Point = Point {
        x: -70.0,
        y: -20.0,
        z: 200.0,
    };
    const R3: f64 = 50.0;
    const COLOUR3: Colour = Colour {
        x: 200,
        y: 0,
        z: 250,
    };
    const TEST_SPHERE3: Sphere = Sphere {
        center: C3,
        radius: R3,
        colour: COLOUR3,
    };

    const C4: Point = Point {
        x: -55.0,
        y: 200.0,
        z: 0.0,
    };
    const R4: f64 = 50.0;
    const COLOUR4: Colour = Colour {
        x: 200,
        y: 200,
        z: 200,
    };
    const TEST_SPHERE4: Sphere = Sphere {
        center: C4,
        radius: R4,
        colour: COLOUR4,
    };

    const C5: Point = Point {
        x: 55.0,
        y: 200.0,
        z: 200.0,
    };
    const R5: f64 = 50.0;
    const COLOUR5: Colour = Colour {
        x: 255,
        y: 255,
        z: 255,
    };
    const TEST_SPHERE5: Sphere = Sphere {
        center: C5,
        radius: R5,
        colour: COLOUR5,
    };

    const TEST_SPHERES: &[Sphere] = &[TEST_SPHERE, TEST_SPHERE2, TEST_SPHERE3, TEST_SPHERE4, TEST_SPHERE5];

    #[test]
    fn it_works() {
        let test_camera = Camera::new(
            Point::new(0.0, 0.0, -300.0),
            Vector3D::new(0.0, 1.0, 0.0),
        );

        let mut test_shapes: Vec<&dyn Shape> = Vec::new();
        TEST_SPHERES.iter().for_each(|s| test_shapes.push(s));

        let mut img = RgbaImage::new(IMG_WIDTH, IMG_HEIGHT);
        let v = Vector::new(125, 255, 125);
        render(&mut img, &test_camera, test_shapes);
        write_img(&img, &Path::new(TEST_PATH));
    }
}
