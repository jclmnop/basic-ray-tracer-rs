use crate::{Point, Vector3D};

pub struct Camera {
    look_at: Point,
    view_reference_point: Point,
    view_plane_normal: Vector3D,
    view_up_vector: Vector3D,
    view_right_vector: Vector3D,
}

// TODO: figure out how to move along x/y/z axes relative to the current
//       VPN by adjusting VRP accordingly
//       e.g. when moving camera up, we don't want to go straight up the
//            y-axis, instead we want to move up in a way that we're rotating
//            around the view_reference_point (0, 0, 0)
impl Camera {
    /// Create a new `Camera` facing the origin (0, 0, 0)
    pub fn new(view_reference_point: Point, approx_view_up_vector: Vector3D) -> Self {
        let look_at = Point::new(0.0, 0.0, 0.0);
        let mut view_plane_normal = (look_at - view_reference_point).to_f64();
        view_plane_normal.normalise();

        let mut view_right_vector = view_plane_normal * approx_view_up_vector;
        view_right_vector.normalise();

        let mut view_up_vector = view_right_vector * view_plane_normal;
        view_up_vector.normalise();

        Self {
            look_at,
            view_reference_point,
            view_plane_normal,
            view_up_vector,
            view_right_vector,
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_created_with_correct_values() {
        let vrp = Point::new(0.0, 0.0, -3.0);
        let vuv = Vector3D::new(0.0, 1.0, 0.0);
        let camera = Camera::new(vrp, vuv);

        assert_eq!(camera.vrp(), vrp);
        assert_eq!(camera.look_at, Point::new(0.0, 0.0, 0.0));
        assert_eq!(camera.vpn(), Vector3D::new(0.0, 0.0, 1.0));
        assert_eq!(camera.vrv(), Vector3D::new(-1.0, 0.0, 0.0));
        assert_eq!(camera.vuv(), vuv);
    }
}
