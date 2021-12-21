use kiss3d::light::Light;
use kiss3d::window::Window;

use nalgebra::Point3;

/// The default name of the canvas
pub static DEFAULT_TITLE: &str = "In Summer We Render";

/// The default width of the canvas
const DEFAULT_WIDTH: u32 = 800u32;

/// The default height of the canvas
const DEFAULT_HEIGHT: u32 = 600u32;

// const WHITE_COLOR: Point3<f32> = Point3::origin();

pub(crate) fn start_window(
    title: Option<&str>,
    width: Option<u32>,
    height: Option<u32>,
    background_color: Option<Point3<f32>>,
) -> Window {
    let mut window = Window::new_with_size(
        title.unwrap_or(DEFAULT_TITLE),
        width.unwrap_or(DEFAULT_WIDTH),
        height.unwrap_or(DEFAULT_HEIGHT),
    );

    window.set_light(Light::StickToCamera);
    let color = background_color.unwrap_or_else(Point3::origin);
    window.set_background_color(color.x / 256.0, color.y / 256.0, color.z / 256.0);

    window
}
