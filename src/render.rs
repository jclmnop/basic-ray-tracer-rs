use std::path::Path;
use image::{Rgba, Pixel, RgbaImage, ImageFormat};
use crate::vector::Vector;
use rayon::prelude::*;
/*
  public void Render(WritableImage image) {
    //Get image dimensions, and declare loop variables
    int w = (int) image.getWidth(), h = (int) image.getHeight(), i, j;
    PixelWriter image_writer = image.getPixelWriter();
    double c = green_col / 255.0;
    Vector col = new Vector(0.5, c, 0.5);
    for (j = 0; j < h; j++) {
      for (i = 0; i < w; i++) {
        image_writer.setColor(i, j, Color.color(col.x, col.y, col.z, 1.0));
      } // column loop
    } // row loop
  }
 */

pub fn render(img: &mut RgbaImage, v: &Vector) {
    img.pixels_mut().par_bridge().into_par_iter().for_each(|mut p| {
        p.0 = [v.x as u8, v.y as u8, v.z as u8, 255];
    });
}

pub fn write_img(img: &RgbaImage, path: &Path) {
    img.save_with_format(path, ImageFormat::Png).expect("It's all gone wrong");
}