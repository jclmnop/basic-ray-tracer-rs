use crate::shapes::{Intersection, Shape, Sphere};
use crate::vector::Vector;
use crate::{Camera, Colour, Point, Ray, Vector3D, IMG_HEIGHT, IMG_WIDTH};
use image::{ImageFormat, Rgba, RgbaImage};
use rayon::prelude::*;
use std::cmp::Ordering;
use std::path::Path;

const BACKGROUND: Colour = Colour { x: 0, y: 0, z: 0 };

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
const COLOUR3: Colour = Colour { x: 200, y: 0, z: 250 };
const TEST_SPHERE3: Sphere = Sphere {
    center: C3,
    radius: R3,
    colour: COLOUR3,
};

const TEST_SPHERES: &[Sphere] = &[TEST_SPHERE, TEST_SPHERE2, TEST_SPHERE3];

// Test scale
const SCALE: f64 = 0.7;

//TODO: render with GDK pixbuf instead (use Relm4?)
pub fn render(img: &mut RgbaImage, v: &Vector<u8>) {
    // TODO: pass camera as arg
    let test_camera = Camera::new(
        Point::new(0.0, 0.0, -300.0),
        Vector3D::new(0.0, 1.0, 0.0),
        SCALE,
    );

    for (j, row) in img.enumerate_rows_mut() {
        row.enumerate()
            .par_bridge()
            .into_par_iter()
            .for_each(|(i, px)| {
                let origin = image_space_point(i, j, &test_camera);
                let ray = Ray {
                    origin,
                    direction: test_camera.vpn(),
                };
                let intersections = TEST_SPHERES
                    .iter()
                    .map(|s| s.intersection(&ray))
                    .collect::<Vec<_>>();
                let closest_intersect = closest_intersect(intersections, &ray.origin);
                let new_colour = if let Some(intersection) = closest_intersect {
                    intersection.1
                } else {
                    BACKGROUND
                };

                px.2 .0 = [new_colour.x, new_colour.y, new_colour.z, 255];
            });
    }
}

pub fn write_img(img: &RgbaImage, path: &Path) {
    img.save_with_format(path, ImageFormat::Png)
        .expect("It's all gone wrong");
}

fn closest_intersect(intersects: Vec<Option<Intersection>>, origin: &Point) -> Option<Intersection> {
    let mut closest: Option<Intersection> = None;
    for intersection_option in intersects {
        if let Some(intersection) = intersection_option {
            if closest.is_none() {
                closest = Some(intersection);
            } else {
                let closest_distance = (*origin - closest.unwrap().0).magnitude().abs();
                let this_distance = (*origin - intersection.0).magnitude().abs();
                closest = match closest_distance.total_cmp(&this_distance) {
                    Ordering::Greater => Some(intersection),
                    _ => closest,
                }
            }
        }
    }
    closest
}

fn image_space_point(i: usize, j: u32, camera: &Camera) -> Point {
    let i = i as f64;
    let j = j as f64;
    let width = IMG_WIDTH as f64;
    let height = IMG_HEIGHT as f64;

    let u = (i - width / 2.0) * camera.scale();
    let v = ((height - j) - height / 2.0) * camera.scale();

    camera.vrp() + (camera.vrv() * u) + (camera.vuv() * v)
}
