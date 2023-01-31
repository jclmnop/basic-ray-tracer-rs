use crate::vector::Vector;
use image::{ImageFormat, Rgba, RgbaImage};
use rayon::prelude::*;
use std::path::Path;

pub fn render(img: &mut RgbaImage, v: &Vector) {
    img.pixels_mut()
        .par_bridge()
        .into_par_iter()
        .for_each(|mut p: &mut Rgba<u8>| {
            p.0 = [v.x as u8, v.y as u8, v.z as u8, 255];
        });
}

pub fn write_img(img: &RgbaImage, path: &Path) {
    img.save_with_format(path, ImageFormat::Png)
        .expect("It's all gone wrong");
}
