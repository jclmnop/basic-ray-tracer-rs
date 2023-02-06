mod camera;
mod lighting;
mod material;
mod render;
mod shapes;
mod vector;

pub use camera::*;
pub use lighting::*;
pub use material::*;
pub use render::*;
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

#[macro_export]
macro_rules! timeit {
        ($code:block) => {{
            let now = ::std::time::Instant::now();
            $code
            let elapsed = now.elapsed();
            elapsed
        }};
    }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::{Shape, Sphere};
    use image::RgbaImage;
    use std::path::Path;

    const TEST_PATH: &str = "./test.png";
    const PIXEL_SCALE: f64 = 0.3;
    const TEST_FRAMES: usize = 100;
    const AMBIENT_COEFFICIENT: f64 = 0.3;

    #[test]
    fn it_works() {
        #![allow(unused_variables)]
        // set_global_rayon_threads(NUM_THREADS);
        let light_source: LightSource = LightSource {
            position: Point::new(-250.0, -250.0, -100.0),
            colour: LightColour::new(1.0, 1.0, 1.0),
        };
        let colour = BURGUNDY;
        let colour2 = BURGUNDY;
        let colour3 = BURGUNDY;
        let material = Material::new(
            colour.to_light_colour() * AMBIENT_COEFFICIENT,
            colour.to_light_colour(),
            colour.to_light_colour(),
            light_source,
        );
        let material2 = Material::new(
            colour2.to_light_colour() * AMBIENT_COEFFICIENT,
            colour2.to_light_colour(),
            colour2.to_light_colour(),
            light_source,
        );
        let material3 = Material::new(
            colour3.to_light_colour() * AMBIENT_COEFFICIENT,
            colour3.to_light_colour(),
            colour3.to_light_colour(),
            light_source,
        );
        let radius: f64 = 49.0;

        let sphere1 = Sphere::new(Point::new(0.0, 0.0, 0.0), radius, material);
        let sphere2 =
            Sphere::new(Point::new(50.0, 50.0, 150.0), radius, material2);
        let sphere3 =
            Sphere::new(Point::new(100.0, 100.0, 300.0), radius, material3);
        let sphere4 =
            Sphere::new(Point::new(-100.0, 0.0, 0.0), radius, material);
        let sphere5 =
            Sphere::new(Point::new(-200.0, 0.0, 0.0), radius, material);
        let sphere6 =
            Sphere::new(Point::new(-300.0, 0.0, 0.0), radius, material);
        let sphere7 =
            Sphere::new(Point::new(0.0, -100.0, 0.0), radius, material);
        let sphere8 =
            Sphere::new(Point::new(0.0, -200.0, 0.0), radius, material);
        let sphere9 =
            Sphere::new(Point::new(0.0, -300.0, 0.0), radius, material);
        let sphere10 =
            Sphere::new(Point::new(-200.0, -200.0, 0.0), radius, material);

        let mut test_shapes: Vec<&dyn Shape> = Vec::new();
        test_shapes.push(&sphere1);
        test_shapes.push(&sphere2);
        test_shapes.push(&sphere3);
        // test_shapes.push(&sphere4);
        // test_shapes.push(&sphere5);
        // test_shapes.push(&sphere6);
        // test_shapes.push(&sphere7);
        // test_shapes.push(&sphere8);
        // test_shapes.push(&sphere9);
        // test_shapes.push(&sphere10);

        let camera_params = CameraParams {
            view_reference_point: Point::new(0.0, 0.0, -1000.0),
            approx_view_up_vector: Vector3D::new(0.0, 1.0, 0.0),
            focal_length: IMG_SIZE as f64,
            img_height: IMG_HEIGHT as usize,
            img_width: IMG_WIDTH as usize,
            scale: PIXEL_SCALE,
        };
        let test_camera = Camera::new(camera_params);

        // let mut test_shapes: Vec<&dyn Shape> = Vec::new();
        // TEST_SPHERES.iter().for_each(|s| test_shapes.push(s));

        let mut img = RgbaImage::new(IMG_WIDTH, IMG_HEIGHT);
        let mut render_times: Vec<u128> = Vec::with_capacity(TEST_FRAMES);
        for _ in 0..TEST_FRAMES {
            let render_time =
                timeit!({ render(&mut img, &test_camera, &test_shapes) })
                    .as_millis();
            render_times.push(render_time);
        }
        let render_time_avg: u128 =
            render_times.iter().sum::<u128>() / render_times.len() as u128;
        let write_time =
            timeit!({ write_img(&img, &Path::new(TEST_PATH)) }).as_millis();
        println!(
            "\
            \n\tImg Size : {IMG_WIDTH}px x {IMG_HEIGHT}px\
            \n\tThreads: {NUM_THREADS}\
            \n\tFrames: {TEST_FRAMES}\n\
            \n\tAvg ms/frame: {render_time_avg}ms\
            \n\tWrite time: {write_time}ms\n\
        "
        );
    }
}
