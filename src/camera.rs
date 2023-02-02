use crate::{Point, Vector3D};

pub struct Camera {
    look_at: Point,
    view_reference_point: Point, // use as projection point
    view_plane_normal: Vector3D, // use to calc position of center pixel in "screen"
    view_up_vector: Vector3D,
    view_right_vector: Vector3D,
    focal_length:f64,
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
    pub scale: f64
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
        let mut view_plane_normal = (look_at - params.view_reference_point).to_f64();
        view_plane_normal.normalise();

        let mut view_right_vector = view_plane_normal * params.approx_view_up_vector;
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
        let mut empty_screen: Vec<Vec<(Point, Vector3D)>> = Vec::with_capacity(self.img_height);
        for _ in 0..empty_screen.capacity() {
            empty_screen.push(Vec::with_capacity(self.img_width));
        }
        self.screen = empty_screen;
        self.setup_screen();
    }

    fn setup_screen(&mut self) {
        let distance_from_projection_point = self.view_plane_normal * self.focal_length;
        let screen_center_point = self.view_reference_point + distance_from_projection_point;
        // let mut screen: Vec<Vec<(Point, Vector3D)>> = Vec::with_capacity(self.img_height);
        for j in 0..self.screen.capacity() {
            // let mut row: Vec<(Point, Vector3D)> = Vec::with_capacity(self.img_width);
            self.screen[j].clear();
            for i in 0..self.screen[j].capacity() {
                let pixel_props = self.calc_pixel_props(i, j, &screen_center_point);
                self.screen[j].push(pixel_props);
            }
        }
    }

    fn calc_pixel_props(&self, i: usize, j: usize, screen_center_point: &Point) -> (Point, Vector3D) {
        let pixel_point = self.calc_pixel_point(i, j, screen_center_point);
        let pixel_direction = self.calc_pixel_direction(&pixel_point);
        (pixel_point, pixel_direction)
    }

    fn calc_pixel_point(&self, i: usize, j: usize, screen_center_point: &Point) -> Point {
        let i = i as f64;
        let j = j as f64;
        let width = self.img_width as f64;
        let height = self.img_height as f64;

        let u = (i - width / 2.0);
        let v = ((height - j) - height / 2.0);

        *screen_center_point + (self.vrv() * u * self.scale) + (self.vuv() * v * self.scale)
    }

    fn calc_pixel_direction(&self, pixel_point: &Point) -> Vector3D {
        *pixel_point - self.view_reference_point
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn camera_created_with_correct_values() {
    //     let vrp = Point::new(0.0, 0.0, -3.0);
    //     let vuv = Vector3D::new(0.0, 1.0, 0.0);
    //     let camera = Camera::new(vrp, vuv);
    //
    //     assert_eq!(camera.vrp(), vrp);
    //     assert_eq!(camera.look_at, Point::new(0.0, 0.0, 0.0));
    //     assert_eq!(camera.vpn(), Vector3D::new(0.0, 0.0, 1.0));
    //     assert_eq!(camera.vrv(), Vector3D::new(-1.0, 0.0, 0.0));
    //     assert_eq!(camera.vuv(), vuv);
    // }
}
