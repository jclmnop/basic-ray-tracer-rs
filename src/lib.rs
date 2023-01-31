mod render;
mod shapes;
mod vector;

pub use render::*;
pub use vector::*;

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbaImage;
    use std::path::Path;
    const TEST_PATH: &str = "./test.png";

    #[test]
    fn it_works() {
        let mut img = RgbaImage::new(640, 640);
        let v = Vector::new(125, 255, 125);
        render(&mut img, &v);
        write_img(&img, &Path::new(TEST_PATH));
    }
}
