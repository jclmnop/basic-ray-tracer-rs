#![allow(dead_code, unused_variables)]
use crate::{LightSource, Point, Vector3D, IMG_HEIGHT, IMG_SIZE, IMG_WIDTH, timeit, Matrix3x3, Vector};
use rayon::prelude::*;

pub struct Camera {
    look_at: Point,
    view_reference_point: Point,
    view_plane_normal: Vector3D,
    view_up_vector: Vector3D,
    view_right_vector: Vector3D,
    focal_length: f64,
    pub screen: Vec<Vec<(Point, Vector3D)>>,
    img_height: usize,
    img_width: usize,
    scale: f64,
    light_source: LightSource,
    fov: f64,
}

pub struct CameraParams {
    pub view_reference_point: Point,
    pub approx_view_up_vector: Vector3D,
    pub focal_length: f64,
    pub img_height: usize,
    pub img_width: usize,
    pub scale: f64,
    pub light_source: LightSource,
    pub fov: f64,
}

impl Default for CameraParams {
    fn default() -> Self {
        Self {
            view_reference_point: Point::new(0.0, 0.0, -(IMG_SIZE as f64) * 1.0),
            approx_view_up_vector: Vector3D::new(0.0, 1.0, 0.0),
            focal_length: 100.0,
            img_height: IMG_HEIGHT as usize,
            img_width: IMG_WIDTH as usize,
            scale: 1.0,
            light_source: LightSource::default(),
            fov: 45.0,
        }
    }
}

/// Just used to pass camera properties to associated functions during parallel
/// iteration, to avoid headaches regarding immutable + mutable references to self
pub struct CameraProps {
    pub screen_center_point: Point,
    pub img_width: usize,
    pub img_height: usize,
    pub scale: f64,
    pub vrv: Vector3D,
    pub vuv: Vector3D,
    pub vrp: Point,
    pub focal_length: f64,
    pub fov: f64,
    pub half_width: f64,
    pub half_height: f64,
    pub pixel_size: f64,
}

//TODO: - store the x/y transform matrices
//      - apply them to original vrp when calling .vrp(), but don't actually
//        mutate vrp field (might have to combine them into one composite rotation
//        matrix or whatever)
impl Camera {
    /// Create a new `Camera` facing the origin (0, 0, 0)
    pub fn new(params: CameraParams) -> Self {
        let look_at = Point::new(0.0, 0.0, 0.0);
        let mut view_plane_normal = look_at - params.view_reference_point;
        view_plane_normal.normalise();

        let mut view_right_vector =
            view_plane_normal * params.approx_view_up_vector;
        view_right_vector.normalise();

        let mut view_up_vector = view_right_vector * view_plane_normal;
        view_up_vector.normalise();

        let mut camera = Self {
            look_at,
            view_reference_point: params.view_reference_point,
            view_plane_normal,
            view_up_vector,
            view_right_vector,
            focal_length: params.focal_length,
            screen: vec![],
            img_height: params.img_height,
            img_width: params.img_width,
            scale: params.scale,
            light_source: LightSource::default(),
            fov: params.fov
        };
        camera.new_screen();

        camera
    }

    pub fn vrp(&self) -> Point {
        self.view_reference_point
    }

    pub fn vpn(&self) -> Vector3D {
        self.view_plane_normal
    }

    pub fn vuv(&self) -> Vector3D {
        self.view_up_vector
    }

    pub fn vrv(&self) -> Vector3D {
        self.view_right_vector
    }

    pub fn light_source(&self) -> LightSource {
        self.light_source
    }

    pub fn set_vrp(&mut self, new_vrp: Point) {
        self.view_reference_point = new_vrp;
        self.adjust_view();
    }

    //TODO: Rotation matrices
    /// Move camera along the x-axis
    pub fn move_x(&mut self, degrees: f64) {
        let rotation_matrix = self.horizontal_rotation_matrix(degrees);
        self.view_reference_point = rotation_matrix * self.view_reference_point;
        self.adjust_view();
    }

    /// Move camera along the y-axis
    pub fn move_y(&mut self, degrees: f64) {
        let rotation_matrix = self.vertical_rotation_matrix(degrees);
        self.view_reference_point = rotation_matrix * self.view_reference_point;
        self.adjust_view();
    }

    pub fn camera_props(&self) -> CameraProps {
        let distance_from_projection_point =
            self.view_plane_normal * self.focal_length;
        let screen_center_point =
            self.view_reference_point + distance_from_projection_point;

        let (half_width, half_height) = self.half_view();

        let pixel_size = self.pixel_size(half_width);

        CameraProps {
            screen_center_point,
            img_width: self.img_width,
            img_height: self.img_height,
            scale: self.scale,
            vrv: self.vrv(),
            vuv: self.vuv(),
            vrp: self.vrp(),
            focal_length: self.focal_length,
            fov: self.fov,
            half_width,
            half_height,
            pixel_size
        }
    }

    fn adjust_view(&mut self) {
        let look_at = Point::new(0.0, 0.0, 0.0);
        self.view_plane_normal = look_at - self.view_reference_point;
        self.view_plane_normal.normalise();

        self.view_right_vector = self.view_plane_normal * self.view_up_vector;
        self.view_right_vector.normalise();

        self.view_up_vector = self.view_right_vector * self.view_plane_normal;
        self.view_up_vector.normalise();

        self.setup_screen();
    }

    fn new_screen(&mut self) {
        let mut empty_screen: Vec<Vec<(Point, Vector3D)>> =
            Vec::with_capacity(self.img_height);
        for _ in 0..empty_screen.capacity() {
            empty_screen.push(Vec::with_capacity(self.img_width));
        }
        self.screen = empty_screen;
        self.setup_screen();
    }

    // TODO: replace with matrix transformations?
    fn setup_screen(&mut self) {
        // This is just a way to get around the issue of passing an immutable
        // reference to self into methods during par_iter_mut()
        let camera_props = self.camera_props();

        //TODO: speed this up by using arrays instead of vectors?
        self.screen.par_iter_mut().enumerate().for_each(|(j, row)| {
            row.clear();
            let capacity = row.capacity();
            for i in 0..capacity {
                let pixel_props = Self::calc_pixel_props(i, j, &camera_props);
                row.push(pixel_props);
            }
        });
    }

    fn aspect_ratio(&self) -> f64 {
        self.img_width as f64 / self.img_height as f64
    }

    /// (half_width, half_height)
    fn half_view(&self) -> (f64, f64) {
        let aspect = self.aspect_ratio();
        // tan(deg) = O / A
        let half_view = (self.fov.to_radians() / 2.0).tan() * self.focal_length;

        if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        }
    }

    fn pixel_size(&self, half_width: f64) -> f64 {
        (half_width * 2.0) / self.img_width as f64
    }

    fn calc_pixel_props(
        i: usize,
        j: usize,
        camera_props: &CameraProps,
    ) -> (Point, Vector3D) {
        let pixel_point = Self::calc_pixel_point(i, j, camera_props);
        let pixel_direction =
            Self::calc_pixel_direction(&camera_props.vrp, &pixel_point);
        (pixel_point, pixel_direction)
    }

    fn calc_pixel_point(
        i: usize,
        j: usize,
        camera_props: &CameraProps,
    ) -> Point {
        let i = i as f64;
        let j = j as f64;
        let width = camera_props.img_width as f64;
        let height = camera_props.img_height as f64;
        let pixel_size = camera_props.pixel_size;
        let scale = camera_props.scale;

        // TODO: I had to invert both of these so that +x is to the right, and
        //       +y is up, what am I doing wrong?
        let u = ((width - i) - width / 2.0) * camera_props.scale * pixel_size;
        let v = ((height - j) - height / 2.0) * camera_props.scale * pixel_size;


        camera_props.screen_center_point
            + (camera_props.vrv * u)
            + (camera_props.vuv * v)
    }

    fn calc_pixel_direction(
        view_reference_point: &Point,
        pixel_point: &Point,
    ) -> Vector3D {
        let mut direction = *pixel_point - *view_reference_point;
        direction.normalise();
        direction
    }

    fn vertical_rotation_matrix(&self, degrees: f64) -> Matrix3x3<f64> {
        let degrees = degrees.to_radians();
        [
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, degrees.cos(), degrees.sin()),
            Vector::new(0.0, -degrees.sin(), degrees.cos())
        ]
    }

    fn horizontal_rotation_matrix(&self, degrees: f64) -> Matrix3x3<f64> {
        let degrees = degrees.to_radians();
        [
            Vector::new(degrees.cos(), 0.0, -degrees.sin()),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(degrees.sin(), 0.0, degrees.cos())
        ]
    }


}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(CameraParams::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::timeit;
    const IMG_HEIGHT: u32 = 1000;
    const IMG_WIDTH: u32 = 1000;
    const PIXEL_SCALE: f64 = 1.0;

    fn test_camera() -> Camera {
        let camera_params = CameraParams {
            view_reference_point: Point::new(0.0, 0.0, -1000.0),
            approx_view_up_vector: Vector3D::new(0.0, 1.0, 0.0),
            focal_length: 1000.0,
            img_height: IMG_HEIGHT as usize,
            img_width: IMG_WIDTH as usize,
            scale: PIXEL_SCALE,
            light_source: LightSource::default(),
            fov: 45.0,
        };
        Camera::new(camera_params)
    }

    #[test]
    fn horizontal_rotation() {
        let mut camera = test_camera();
        camera.move_x(10.0);
        assert_eq!(0.0, camera.view_reference_point.y);
    }

    //
    // #[test]
    // fn camera_created_with_correct_values() {
    //     let camera = test_camera();
    //
    //     assert_eq!(camera.vrp(), Point::new(0.0, 0.0, -1000.0));
    //     assert_eq!(camera.look_at, Point::new(0.0, 0.0, 0.0));
    //     assert_eq!(camera.vpn(), Vector3D::new(0.0, 0.0, 1.0));
    //     assert_eq!(camera.vrv(), Vector3D::new(-1.0, 0.0, 0.0));
    //     assert_eq!(camera.vuv(), Vector3D::new(0.0, 1.0, 0.0));
    //
    //     let setup_time = timeit!({
    //         test_camera();
    //     })
    //     .as_millis();
    //     println!("\tCamera setup time: {setup_time}ms");
    // }
    //
    // #[test]
    // fn correct_pixel_point() {
    //     let camera = test_camera();
    //     let distance_from_projection_point =
    //         camera.view_plane_normal * camera.focal_length;
    //     let screen_center_point =
    //         camera.view_reference_point + distance_from_projection_point;
    //     let i_center = camera.img_width / 2;
    //     let j_center = camera.img_height / 2;
    //     let props = CameraProps {
    //         screen_center_point,
    //         img_width: camera.img_width,
    //         img_height: camera.img_height,
    //         scale: camera.scale,
    //         vrv: camera.vrv(),
    //         vuv: camera.vuv(),
    //         vrp: camera.vrp(),
    //         focal_length: camera.focal_length,
    //     };
    //     let point_center_pixel =
    //         Camera::calc_pixel_point(i_center, j_center, &props);
    //     let (i_left, j_left) = (i_center - 300, j_center);
    //     let point_left_pixel = Camera::calc_pixel_point(i_left, j_left, &props);
    //
    //     let (i_down, j_down) = (i_center, j_center + 300);
    //     let point_down_pixel = Camera::calc_pixel_point(i_down, j_down, &props);
    //
    //     let (i_diagonal, j_diagonal) = (i_center - 300, j_center + 300);
    //     let point_diagonal_pixel =
    //         Camera::calc_pixel_point(i_diagonal, j_diagonal, &props);
    //
    //     println!();
    //     println!("\tcentre: {}", point_center_pixel);
    //     println!("\tleft: {point_left_pixel}");
    //     println!("\tdown: {point_down_pixel}");
    //     println!("\tdiagonal: {point_diagonal_pixel}");
    //     println!("\tVRP: {}\n\tVPN: {}", camera.vrp(), camera.vpn());
    //     println!();
    //
    //     assert_eq!(point_center_pixel, screen_center_point);
    //     assert_eq!(point_left_pixel, Point::new(-300.0, 0.0, 0.0));
    //     assert_eq!(point_down_pixel, Point::new(0.0, -300.0, 0.0));
    //     assert_eq!(point_diagonal_pixel, Point::new(-300.0, -300.0, 0.0));
    // }
}
