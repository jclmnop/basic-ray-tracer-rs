#![allow(dead_code, unused_variables)]
use crate::{Point, Vector3D};
use rayon::prelude::*;

pub struct Camera {
    look_at: Point,
    view_reference_point: Point, // use as projection point
    view_plane_normal: Vector3D, // use to calc position of center pixel in "screen"
    view_up_vector: Vector3D,
    view_right_vector: Vector3D,
    focal_length: f64,
    pub screen: Vec<Vec<(Point, Vector3D)>>,
    img_height: usize,
    img_width: usize,
    scale: f64,
}

pub struct CameraParams {
    pub view_reference_point: Point,
    pub approx_view_up_vector: Vector3D,
    pub focal_length: f64,
    pub img_height: usize,
    pub img_width: usize,
    pub scale: f64,
}

// TODO: figure out how to move along x/y/z axes relative to the current
//       VPN by adjusting VRP accordingly
//       e.g. when moving camera up, we don't want to go straight up the
//            y-axis, instead we want to move up in a way that we're rotating
//            around the view_reference_point (0, 0, 0)
//TODO: replace orthogonal ray tracing (and scaling etc) with perspective raytracing
//          - add focal length
//          - add "view plane" (screen, 2D array)
//          - calc ray direction from each pixel in view plane, and store in array
//          - `(pixel_point - projection_point).normalise() * -1.0`
//          - use that direction in render() function for each pixel
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
        let distance_from_projection_point =
            self.view_plane_normal * self.focal_length;
        let screen_center_point =
            self.view_reference_point + distance_from_projection_point;
        let img_width = self.img_width;
        let img_height = self.img_height;
        let scale = self.scale;
        let vrv = self.vrv();
        let vuv = self.vuv();
        let vrp = self.vrp();

        //TODO: speed this up by using arrays instead of vectors?
        self.screen.par_iter_mut().enumerate().for_each(|(j, row)| {
            row.clear();
            let capacity = row.capacity();
            for i in 0..capacity {
                let pixel_props = Self::calc_pixel_props(
                    i,
                    j,
                    &screen_center_point,
                    img_width,
                    img_height,
                    scale,
                    vrv,
                    vuv,
                    vrp,
                );
                row.push(pixel_props);
            }
        });
    }

    fn calc_pixel_props(
        i: usize,
        j: usize,
        screen_center_point: &Point,
        img_width: usize,
        img_height: usize,
        scale: f64,
        vrv: Vector3D,
        vuv: Vector3D,
        vrp: Point,
    ) -> (Point, Vector3D) {
        let pixel_point = Self::calc_pixel_point(
            i,
            j,
            screen_center_point,
            img_width,
            img_height,
            scale,
            vrv,
            vuv,
        );
        let pixel_direction = Self::calc_pixel_direction(&vrp, &pixel_point);
        (pixel_point, pixel_direction)
    }

    fn calc_pixel_point(
        i: usize,
        j: usize,
        screen_center_point: &Point,
        img_width: usize,
        img_height: usize,
        scale: f64,
        vrv: Vector3D,
        vuv: Vector3D,
    ) -> Point {
        let i = i as f64;
        let j = j as f64;
        let width = img_width as f64;
        let height = img_height as f64;

        // TODO: I had to invert both of these so that +x is to the right, and
        //       +y is up, what am I doing wrong?
        let u = ((width - i) - width / 2.0) * scale;
        let v = ((height - j) - height / 2.0) * scale;

        *screen_center_point + (vrv * u) + (vuv * v)
    }

    fn calc_pixel_direction(
        view_reference_point: &Point,
        pixel_point: &Point,
    ) -> Vector3D {
        *view_reference_point - *pixel_point
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
            view_reference_point: Point::new(0.0, 0.0, -200.0),
            approx_view_up_vector: Vector3D::new(0.0, 1.0, 0.0),
            focal_length: 200.0,
            img_height: IMG_HEIGHT as usize,
            img_width: IMG_WIDTH as usize,
            scale: PIXEL_SCALE,
        };
        Camera::new(camera_params)
    }

    #[test]
    fn camera_created_with_correct_values() {
        let camera = test_camera();

        assert_eq!(camera.vrp(), Point::new(0.0, 0.0, -200.0));
        assert_eq!(camera.look_at, Point::new(0.0, 0.0, 0.0));
        assert_eq!(camera.vpn(), Vector3D::new(0.0, 0.0, 1.0));
        assert_eq!(camera.vrv(), Vector3D::new(-1.0, 0.0, 0.0));
        assert_eq!(camera.vuv(), Vector3D::new(0.0, 1.0, 0.0));

        let setup_time = timeit!({
            test_camera();
        })
        .as_millis();
        println!("Camera setup time: {setup_time}ms");
    }

    // #[test]
    // fn correct_pixel_point() {
    //     let camera = test_camera();
    //     let distance_from_projection_point =
    //         camera.view_plane_normal * camera.focal_length;
    //     let screen_center_point =
    //         camera.view_reference_point + distance_from_projection_point;
    //     let i_center = camera.img_width / 2;
    //     let j_center = camera.img_height / 2;
    //     let point_center_pixel =
    //         camera.calc_pixel_point(i_center, j_center, &screen_center_point);
    //
    //     let (i_left, j_left) = (i_center - 300, j_center);
    //     let point_left_pixel =
    //         camera.calc_pixel_point(i_left, j_left, &screen_center_point);
    //
    //     let (i_down, j_down) = (i_center, j_center + 300);
    //     let point_down_pixel =
    //         camera.calc_pixel_point(i_down, j_down, &screen_center_point);
    //
    //     let (i_diagonal, j_diagonal) = (i_center - 300, j_center + 300);
    //     let point_diagonal_pixel = camera.calc_pixel_point(
    //         i_diagonal,
    //         j_diagonal,
    //         &screen_center_point,
    //     );
    //
    //     println!();
    //     println!("centre: {}", point_center_pixel);
    //     println!("left: {point_left_pixel}");
    //     println!("down: {point_down_pixel}");
    //     println!("diagonal: {point_diagonal_pixel}");
    //     println!();
    //
    //     assert_eq!(point_center_pixel, screen_center_point);
    //     assert_eq!(point_left_pixel, Point::new(-300.0, 0.0, 0.0));
    //     assert_eq!(point_down_pixel, Point::new(0.0, -300.0, 0.0));
    //     assert_eq!(point_diagonal_pixel, Point::new(-300.0, -300.0, 0.0));
    // }
}
