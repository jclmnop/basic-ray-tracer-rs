mod camera;
mod lighting;
mod material;
mod render;
mod shapes;
mod vector;

pub use camera::*;
use image::RgbaImage;
pub use lighting::*;
pub use material::*;
pub use render::*;
pub use shapes::*;
pub use vector::*;

// Image parameters TODO: ImageParam struct
pub const IMG_SIZE: u32 = 1000;
pub const IMG_HEIGHT: u32 = IMG_SIZE;
pub const IMG_WIDTH: u32 = IMG_SIZE;

// Threads
pub const NUM_THREADS: usize = 10;

pub fn set_global_rayon_threads(n: usize) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(n)
        .build_global()
        .unwrap();
}

pub fn black_img(img: &mut RgbaImage) {
    img.pixels_mut().for_each(|mut p| {
        let black = PixelColour::default();
        p.0 = [black.x, black.y, black.z, 255];
    });
}

#[macro_export]
macro_rules! timeit {
        ($code:block) => {{
            let now = ::std::time::Instant::now();
            $code
            let elapsed = now.elapsed();
            elapsed
        }};
    }
