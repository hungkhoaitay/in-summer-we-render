use crate::errors::*;
use crate::ply::Ply;
use crate::ply_dir::PlyDir;
use crate::points::Points;
use nalgebra::Point3;

pub trait UI {
    fn start(
        &mut self,
        title: Option<&str>,
        width: Option<u32>,
        height: Option<u32>,
        background_color: Option<Point3<f32>>,
        eye: Option<Point3<f32>>,
        at: Option<Point3<f32>>,
    );

    /// Open the window and render the frame many times
    fn render_image(&mut self, data: &Points);

    fn render_video(&mut self, ply_dir: PlyDir) -> Result<()>;

    fn change_info_bar_state(&mut self);

    /// Render a ply file to png format
    ///
    /// # Arguments
    /// * `x` - the x-coordinate of the bottom left corner
    /// * `y` - the y-coordinate of the bottom left corner
    /// * `width` - the width of the png
    /// * `height` - the height of the png
    /// * `path` - the path to save the png file
    fn save_to_png(
        &mut self,
        ply: &mut Ply,
        x: Option<u32>,
        y: Option<u32>,
        width: Option<u32>,
        height: Option<u32>,
        output: Option<&str>,
    ) -> Result<()>;
}
