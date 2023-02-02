use crate::shapes::{Intersection, Shape};
use crate::{Camera, Colour, Point, Ray, IMG_HEIGHT, IMG_WIDTH};
use image::{ImageFormat, RgbaImage};
use rayon::prelude::*;
use std::cmp::Ordering;
use std::path::Path;

const BACKGROUND: Colour = Colour { x: 0, y: 0, z: 0 };

//TODO: replace orthogonal ray tracing (and scaling etc) with perspective raytracing

//TODO: render with GDK pixbuf instead (use Relm4?)
pub fn render(img: &mut RgbaImage, camera: &Camera, shapes: Vec<&dyn Shape>) {
    for (j, row) in img.enumerate_rows_mut() {
        row.enumerate()
            .par_bridge()
            .into_par_iter()
            .for_each(|(i, px)| {
                let new_colour = calculate_pixel_colour(i, j, camera, &shapes);
                px.2 .0 = [new_colour.x, new_colour.y, new_colour.z, 255];
            });
    }
}

pub fn write_img(img: &RgbaImage, path: &Path) {
    img.save_with_format(path, ImageFormat::Png)
        .expect("It's all gone wrong");
}

fn calculate_pixel_colour(i: usize, j: u32, camera: &Camera, shapes: &Vec<&dyn Shape>) -> Colour {
    let (origin, direction) = camera.screen[j as usize][i];
    let ray = Ray {
        origin,
        direction,
    };
    let intersections = shapes
        .iter()
        .map(|s| s.intersection(&ray))
        .collect::<Vec<_>>();
    let closest_intersect = closest_intersect(intersections, &ray.origin);
    if let Some(intersection) = closest_intersect {
        intersection.1
    } else {
        BACKGROUND
    }
}

fn closest_intersect(
    intersects: Vec<Option<Intersection>>,
    origin: &Point,
) -> Option<Intersection> {
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

    let u = (i - width / 2.0);
    let v = ((height - j) - height / 2.0);

    camera.vrp() + (camera.vrv() * u) + (camera.vuv() * v)
}
