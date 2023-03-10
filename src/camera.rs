use crate::{
    matrix_mul, LightSource, Matrix3x3, Point, Vector, Vector3D, IMG_HEIGHT,
    IMG_SIZE, IMG_WIDTH,
};
use rayon::prelude::*;

const APPROX_VUV: Vector3D = Vector {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

const DEFAULT_AMBIENT_COEFFICIENT: f64 = 0.3;

pub struct Camera {
    look_at: Point,
    view_reference_point: Point,
    view_plane_normal: Vector3D,
    view_up_vector: Vector3D,
    view_right_vector: Vector3D,
    focal_length: f64,
    screen: Vec<Vec<(Point, Vector3D)>>,
    ambient_coefficient: f64,
    img_height: usize,
    img_width: usize,
    scale: f64,
    pub light_source: LightSource,
    fov: f64,
    h_rotation: f64,
    v_rotation: f64,
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
    pub ambient_coefficient: f64,
}

impl Default for CameraParams {
    fn default() -> Self {
        Self {
            view_reference_point: Point::new(
                0.0,
                0.0,
                -(IMG_SIZE as f64) * 1.0,
            ),
            approx_view_up_vector: APPROX_VUV,
            focal_length: 100.0,
            img_height: IMG_HEIGHT as usize,
            img_width: IMG_WIDTH as usize,
            scale: 1.0,
            light_source: LightSource::default(),
            fov: 45.0,
            ambient_coefficient: DEFAULT_AMBIENT_COEFFICIENT,
        }
    }
}

/// Just used to pass camera properties to associated functions during parallel
/// iteration, to avoid headaches regarding immutable + mutable references to self
#[derive(Debug)]
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
            fov: params.fov,
            h_rotation: 0.0,
            v_rotation: 0.0,
            ambient_coefficient: params.ambient_coefficient,
        };
        camera.new_screen();
        // println!("h: {}, v: {}", camera.h_rotation, camera.v_rotation);
        // println!("BEFORE:\n{:#?}", camera.camera_props());

        camera
    }

    pub fn vrp(&self) -> Point {
        self.general_rotation_matrix() * self.view_reference_point
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

    pub fn ambient_coefficient(&self) -> f64 {
        self.ambient_coefficient
    }

    pub fn set_ambient_coefficient(&mut self, new_value: f64) {
        let new_value = if new_value > 1.0 {
            1.0
        } else if new_value < 0.0 {
            0.0
        } else {
            new_value
        };

        self.ambient_coefficient = new_value;
    }

    pub fn light_source(&self) -> LightSource {
        self.light_source
    }

    pub fn reset_vrp(&mut self) {
        self.view_up_vector = APPROX_VUV;
        self.h_rotation = 0.0;
        self.v_rotation = 0.0;
        self.adjust_view();
    }

    pub fn reset_x(&mut self) {
        self.h_rotation = 0.0;
        self.adjust_view();
    }

    pub fn reset_y(&mut self) {
        self.view_up_vector = APPROX_VUV;
        self.v_rotation = 0.0;
        self.adjust_view();
    }

    /// Move camera along the x-axis
    pub fn move_x(&mut self, degrees: f64) {
        self.h_rotation += degrees;
        if self.h_rotation > 360.0 {
            self.h_rotation -= 360.0;
        } else if self.h_rotation < 0.0 {
            self.h_rotation += 360.0;
        }
        self.adjust_view();
    }

    /// Move camera along the y-axis
    pub fn move_y(&mut self, degrees: f64) {
        self.v_rotation += degrees;
        if self.v_rotation > 90.0 {
            self.v_rotation = 90.0;
        } else if self.v_rotation < -90.0 {
            self.v_rotation = -90.0;
        }
        self.adjust_view();
    }

    pub fn h_rotation(&self) -> f64 {
        self.h_rotation
    }

    pub fn v_rotation(&self) -> f64 {
        self.v_rotation
    }

    pub fn pixel_props(
        &self,
        i: usize,
        j: usize,
        rotation_matrix: &Matrix3x3<f64>,
    ) -> (Point, Vector3D) {
        let (origin, direction) = self.screen[j as usize][i];
        (rotation_matrix * origin, rotation_matrix * direction)
    }

    pub fn camera_props(&self) -> CameraProps {
        let distance_from_projection_point =
            self.view_plane_normal * self.focal_length;
        let screen_center_point = self.vrp() + distance_from_projection_point;

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
            pixel_size,
        }
    }

    pub fn general_rotation_matrix(&self) -> Matrix3x3<f64> {
        matrix_mul(
            self.horizontal_rotation_matrix(),
            self.vertical_rotation_matrix(),
        )
    }

    fn adjust_view(&mut self) {
        let look_at = self.look_at;
        let approx_view_up = self.view_up_vector;
        self.view_plane_normal = look_at - self.vrp();
        self.view_plane_normal.normalise();

        self.view_right_vector = self.view_plane_normal * approx_view_up;
        self.view_right_vector.normalise();

        self.view_up_vector = self.view_right_vector * self.view_plane_normal;
        self.view_up_vector.normalise();
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
        let u = ((width - i) - width / 2.0) * scale * pixel_size;
        let v = ((height - j) - height / 2.0) * scale * pixel_size;

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

    fn vertical_rotation_matrix(&self) -> Matrix3x3<f64> {
        let degrees = self.v_rotation.to_radians();
        [
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, degrees.cos(), degrees.sin()),
            Vector::new(0.0, -degrees.sin(), degrees.cos()),
        ]
    }

    fn horizontal_rotation_matrix(&self) -> Matrix3x3<f64> {
        let degrees = self.h_rotation.to_radians();
        [
            Vector::new(degrees.cos(), 0.0, -degrees.sin()),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(degrees.sin(), 0.0, degrees.cos()),
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
            ambient_coefficient: DEFAULT_AMBIENT_COEFFICIENT,
        };
        Camera::new(camera_params)
    }

    #[test]
    fn horizontal_rotation() {
        let mut camera = test_camera();
        camera.move_x(10.0);
        assert_eq!(0.0, camera.vrp().y);
    }
}
