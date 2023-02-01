use crate::vector::Vector;
use image::{ImageFormat, Rgba, RgbaImage};
use rayon::prelude::*;
use std::path::Path;
use crate::{Camera, Colour, Point, Vector3D};
use crate::shapes::Sphere;

const BACKGROUND: Colour = Colour {x: 0, y: 0, z: 0};

// Test values
const C: Point = Point {x: 0.0, y: 0.0, z: 0.0};
const R: f64 = 3.0;
const COLOUR: Colour = Colour {x: 100, y: 200, z: 100};
const TEST_SPHERE: Sphere = Sphere {center: C, radius: R, colour: COLOUR};

//TODO: render basic circle
//TODO: render with GDK pixbuf instead (use Relm4?)

pub fn render(img: &mut RgbaImage, v: &Vector<u8>) {
    let test_camera = Camera::new(
        Point::new(0.0, 0.0, -200.0),
        Vector3D::new(0.0, 1.0, 0.0),
        1.0
    );

    //TODO: impl viewspace + pixel scaling
    img.pixels_mut()
        .par_bridge()
        .into_par_iter()
        .for_each(|mut p: &mut Rgba<u8>| {
            p.0 = [v.x, v.y, v.z, 255];
        });
}

pub fn write_img(img: &RgbaImage, path: &Path) {
    img.save_with_format(path, ImageFormat::Png)
        .expect("It's all gone wrong");
}
