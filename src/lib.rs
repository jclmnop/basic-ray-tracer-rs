mod vector;
mod render;
mod sphere;

pub use vector::*;
pub use render::*;

#[cfg(test)]
mod tests {
    use std::path::Path;
    use image::RgbaImage;
    use super::*;
    const TEST_PATH: &str = "./test.png";

    #[test]
    fn it_works() {
        let mut img = RgbaImage::new(640, 640);
        let v = Vector::new(0.5 * 255.0, 255.0, 0.5 * 255.0);
        render(&mut img, &v);
        write_img(&img, &Path::new(TEST_PATH));
    }
}
