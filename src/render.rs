use crate::vector::Vector;
use image::{ImageFormat, Rgba, RgbaImage};
use rayon::prelude::*;
use std::path::Path;

pub fn render(img: &mut RgbaImage, v: &Vector<u8>) {
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
