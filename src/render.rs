use crate::shapes::Shape;
use crate::{Camera, Intersection, PixelColour, Point, Ray, Sphere};
use image::{ImageFormat, RgbaImage};
use rayon::prelude::*;
use std::cmp::Ordering;
use std::path::Path;

const BACKGROUND: PixelColour = PixelColour { x: 0, y: 0, z: 0 };

//TODO: render with GDK pixbuf instead (use Relm4?)
pub fn render(img: &mut RgbaImage, camera: &Camera, shapes: &Vec<Sphere>) {
    img.enumerate_rows_mut()
        .par_bridge()
        .into_par_iter()
        .for_each(|(j, row)| {
            row.enumerate()
                // .par_bridge()
                // .into_par_iter()
                .for_each(|(i, px)| {
                    let new_colour =
                        calculate_pixel_colour(i, j, camera, &shapes);
                    let new_colour =
                        [new_colour.x, new_colour.y, new_colour.z, 255];
                    if new_colour != px.2 .0 {
                        px.2 .0 = new_colour;
                    }
                });
        })
}

pub fn write_img(img: &RgbaImage, path: &Path) {
    img.save_with_format(path, ImageFormat::Png)
        .expect("It's all gone wrong");
}

fn calculate_pixel_colour(
    i: usize,
    j: u32,
    camera: &Camera,
    shapes: &Vec<Sphere>,
) -> PixelColour {
    let (origin, direction) = camera.screen[j as usize][i];
    let ray = Ray { origin, direction };
    let intersections = shapes
        .iter()
        .map(|s| s.intersection(&ray, &camera))
        .collect::<Vec<_>>();
    let closest_intersect = closest_intersect(intersections);
    if let Some(intersection) = closest_intersect {
        intersection.phong(&origin)
    } else {
        BACKGROUND
    }
}

fn closest_intersect(
    intersects: Vec<Option<Intersection>>,
) -> Option<Intersection> {
    let mut closest: Option<Intersection> = None;
    for intersection_option in intersects {
        if let Some(intersection) = intersection_option {
            if let Some(closest_intersection) = closest {
                match intersection.t().total_cmp(&closest_intersection.t()) {
                    Ordering::Less => closest = Some(intersection),
                    _ => {}
                }
            } else {
                closest = Some(intersection);
            }
        }
    }
    closest
}
