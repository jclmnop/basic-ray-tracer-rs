#![allow(dead_code, unused_variables)]
use crate::{LightSource, Point, Vector3D, IMG_HEIGHT, IMG_SIZE, IMG_WIDTH};
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
}

pub struct CameraParams {
    pub view_reference_point: Point,
    pub approx_view_up_vector: Vector3D,
    pub focal_length: f64,
    pub img_height: usize,
    pub img_width: usize,
    pub scale: f64,
    pub light_source: LightSource,
}

impl Default for CameraParams {
    fn default() -> Self {
        Self {
            view_reference_point: Point::new(0.0, 0.0, -(IMG_SIZE as f64)),
            approx_view_up_vector: Vector3D::new(0.0, 1.0, 0.0),
            focal_length: IMG_SIZE as f64,
            img_height: IMG_HEIGHT as usize,
            img_width: IMG_WIDTH as usize,
            scale: 1.0,
            light_source: LightSource::default(),
        }
    }
}

pub struct CameraProps {
    pub screen_center_point: Point,
    pub img_width: usize,
    pub img_height: usize,
    pub scale: f64,
    pub vrv: Vector3D,
    pub vuv: Vector3D,
    pub vrp: Point,
}

// TODO: figure out how to move along x/y/z axes relative to the current
//       VPN by adjusting VRP accordingly
//       e.g. when moving camera up, we don't want to go straight up the
//            y-axis, instead we want to move up in a way that we're rotating
//            around the view_reference_point (0, 0, 0)
impl Camera {
    /// Create a new `Camera` facing the origin (0, 0, 0)
    pub fn new(params: CameraParams) -> Self {
        let look_at = Point::new(0.0, 0.0, 0.0);
        let mut view_plane_normal =
            (look_at - params.view_reference_point).to_f64();
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

    /// Move camera along the x-axis
    pub fn move_x(&mut self, delta: i64) {
        todo!()
    }

    /// Move camera along the y-axis
    pub fn move_y(&mut self, delta: i64) {
        todo!()
    }

    /// Move camera along the z-axis
    pub fn move_z(&mut self, delta: i64) {
        todo!()
    }

    pub fn camera_props(&self) -> CameraProps {
        let distance_from_projection_point =
            self.view_plane_normal * self.focal_length;
        let screen_center_point =
            self.view_reference_point + distance_from_projection_point;

        CameraProps {
            screen_center_point,
            img_width: self.img_width,
            img_height: self.img_height,
            scale: self.scale,
            vrv: self.vrv(),
            vuv: self.vuv(),
            vrp: self.vrp(),
        }
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

        // TODO: I had to invert both of these so that +x is to the right, and
        //       +y is up, what am I doing wrong?
        let u = ((width - i) - width / 2.0) * camera_props.scale;
        let v = ((height - j) - height / 2.0) * camera_props.scale;

        camera_props.screen_center_point
            + (camera_props.vrv * u)
            + (camera_props.vuv * v)
    }

    fn calc_pixel_direction(
        view_reference_point: &Point,
        pixel_point: &Point,
    ) -> Vector3D {
        *view_reference_point - *pixel_point
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
        };
        Camera::new(camera_params)
    }

    #[test]
    fn camera_created_with_correct_values() {
        let camera = test_camera();

        assert_eq!(camera.vrp(), Point::new(0.0, 0.0, -1000.0));
        assert_eq!(camera.look_at, Point::new(0.0, 0.0, 0.0));
        assert_eq!(camera.vpn(), Vector3D::new(0.0, 0.0, 1.0));
        assert_eq!(camera.vrv(), Vector3D::new(-1.0, 0.0, 0.0));
        assert_eq!(camera.vuv(), Vector3D::new(0.0, 1.0, 0.0));

        let setup_time = timeit!({
            test_camera();
        })
        .as_millis();
        println!("\tCamera setup time: {setup_time}ms");
    }

    #[test]
    fn correct_pixel_point() {
        let camera = test_camera();
        let distance_from_projection_point =
            camera.view_plane_normal * camera.focal_length;
        let screen_center_point =
            camera.view_reference_point + distance_from_projection_point;
        let i_center = camera.img_width / 2;
        let j_center = camera.img_height / 2;
        let props = CameraProps {
            screen_center_point,
            img_width: camera.img_width,
            img_height: camera.img_height,
            scale: camera.scale,
            vrv: camera.vrv(),
            vuv: camera.vuv(),
            vrp: camera.vrp(),
        };
        let point_center_pixel =
            Camera::calc_pixel_point(i_center, j_center, &props);
        let (i_left, j_left) = (i_center - 300, j_center);
        let point_left_pixel = Camera::calc_pixel_point(i_left, j_left, &props);

        let (i_down, j_down) = (i_center, j_center + 300);
        let point_down_pixel = Camera::calc_pixel_point(i_down, j_down, &props);

        let (i_diagonal, j_diagonal) = (i_center - 300, j_center + 300);
        let point_diagonal_pixel =
            Camera::calc_pixel_point(i_diagonal, j_diagonal, &props);

        println!();
        println!("\tcentre: {}", point_center_pixel);
        println!("\tleft: {point_left_pixel}");
        println!("\tdown: {point_down_pixel}");
        println!("\tdiagonal: {point_diagonal_pixel}");
        println!("\tVRP: {}\n\tVPN: {}", camera.vrp(), camera.vpn());
        println!();

        assert_eq!(point_center_pixel, screen_center_point);
        assert_eq!(point_left_pixel, Point::new(-300.0, 0.0, 0.0));
        assert_eq!(point_down_pixel, Point::new(0.0, -300.0, 0.0));
        assert_eq!(point_diagonal_pixel, Point::new(-300.0, -300.0, 0.0));
    }
}
