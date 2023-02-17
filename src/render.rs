use crate::shapes::Shape;
use crate::{
    Camera, Intersection, Matrix3x3, PixelColour, Ray, Sphere,
    ARRAY_WIDTH, BYTES_PER_PIXEL,
};
use gtk::gdk_pixbuf::Pixbuf;
use image::{ImageFormat, RgbaImage};
use rayon::prelude::*;
use std::cmp::Ordering;
use std::path::Path;

const BACKGROUND: PixelColour = PixelColour { x: 0, y: 0, z: 0 };

pub fn render(img: &mut Pixbuf, camera: &Camera, shapes: &Vec<Sphere>) {
    let rotation_matrix = camera.general_rotation_matrix();
    // Unsafe because of Pixbuf.pixels(), should be fine though because the reason
    // unsafe is because you can't have any other reads/writes to the pixbuf while
    // the pixels() reference is still active, and because this is all taking
    // place inside a function where Pixbuf has been passed as &mut reference,
    // nothing else can read/write Pixbuf anyway.
    unsafe {
        img.pixels()
            .par_chunks_mut(ARRAY_WIDTH)
            .enumerate()
            .for_each(|(j, row)| {
                row.chunks_mut(BYTES_PER_PIXEL).enumerate().for_each(
                    |(i, px)| {
                        let new_colour = calculate_pixel_colour(
                            i,
                            j,
                            camera,
                            &shapes,
                            &rotation_matrix,
                        );
                        let new_colour =
                            [new_colour.x, new_colour.y, new_colour.z, 255];
                        if new_colour != px {
                            px.copy_from_slice(&new_colour);
                        }
                    },
                );
            })
    }
}

pub fn write_img(img: &RgbaImage, path: &Path) {
    img.save_with_format(path, ImageFormat::Png)
        .expect("It's all gone wrong");
}

fn calculate_pixel_colour(
    i: usize,
    j: usize,
    camera: &Camera,
    shapes: &Vec<Sphere>,
    rotation_matrix: &Matrix3x3<f64>,
) -> PixelColour {
    let (origin, direction) = camera.pixel_props(i, j, rotation_matrix);
    let ray = Ray { origin, direction };
    let intersections = shapes
        .iter()
        .map(|s| s.intersection(&ray, &camera))
        .collect::<Vec<_>>();
    let closest_intersect = closest_intersect(intersections);
    if let Some(intersection) = closest_intersect {
        intersection.phong(&origin, camera.ambient_coefficient())
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
