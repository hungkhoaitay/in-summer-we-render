use kiss3d::camera::ArcBall;
use nalgebra::Point3;

const DEFAULT_EYE: Point3<f32> = Point3::new(0.0f32, 500.0, 1969.0);
const DEFAULT_AT: Point3<f32> = Point3::new(300.0f32, 500.0, 200.0);

pub(crate) fn start_camera(eye: Option<Point3<f32>>, at: Option<Point3<f32>>) -> ArcBall {
    ArcBall::new_with_frustrum(
        std::f32::consts::PI / 4.0,
        0.1,
        10000.0,
        eye.unwrap_or(DEFAULT_EYE),
        at.unwrap_or(DEFAULT_AT),
    )
}
