mod camera;
mod ray;
mod render;
mod shapes;
mod vector;

pub use camera::*;
pub use ray::*;
pub use render::*;
pub use vector::*;

pub const IMG_HEIGHT: u32 = 1000;
pub const IMG_WIDTH: u32 = 1000;

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbaImage;
    use std::path::Path;
    const TEST_PATH: &str = "./test.png";

    #[test]
    fn it_works() {
        let mut img = RgbaImage::new(IMG_WIDTH, IMG_HEIGHT);
        let v = Vector::new(125, 255, 125);
        render(&mut img, &v);
        write_img(&img, &Path::new(TEST_PATH));
    }
}
