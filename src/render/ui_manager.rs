 use crate::errors::*;

use crate::logic::ui_controller::UIController;
use crate::render::ui::UI;

use crate::ply::Ply;
use crate::ply_dir::PlyDir;
use crate::points::Points;
use nalgebra::Point3;

use kiss3d::window::Window;
use kiss3d::camera::ArcBall;

use crate::render::window_producer;
use crate::render::camera_producer;
use crate::render::frame_renderer;
use crate::render::video_renderer;

// use crate::gui::gui::{ gui, Ids, InfoBar };

pub struct UIManager {
    ui_controller: Box<dyn UIController>,
    window: Option<Window>,
    camera: Option<ArcBall>,
}

impl UIManager {
    pub fn new(ui_controller: Box<dyn UIController>) -> Box<dyn UI> where Self: Sized {
        Box::new(UIManager {
            ui_controller: ui_controller,
            window: None,
            camera: None,
        })
    }

    fn require_window_not_null(&self) {
        assert!(self.window.is_some());
    }

    fn require_camera_not_null(&self) {
        assert!(self.camera.is_some());
    }

    /// Render with default camera
    pub fn render(&mut self) -> bool {
        self.window.as_mut().unwrap().render_with_camera(self.camera.as_mut().unwrap())
    }

    pub (crate) fn draw_point_with_size(&mut self, point: &Point3<f32>, color: &Point3<f32>, size: f32) {
        self.window.as_mut().unwrap().draw_point_with_size(point, color, size);
    }

    // pub fn render_frame(&mut self, data: &Points, ids: &Ids, app: &mut InfoBar) {
    //     frame_renderer::render_frame(self, data, ids, app);
    // }

    pub fn render_frame(&mut self, data: &Points) {
        // frame_renderer::render_frame(self, data, ids, app);
        frame_renderer::render_frame(self, data);
    }
}


impl UI for UIManager {
    fn start(
        &mut self,
        title: Option<&str>,
        width: Option<u32>,
        height: Option<u32>,
        background_color: Option<Point3<f32>>,
        eye: Option<Point3<f32>>,
        at: Option<Point3<f32>>,
    ) {
        self.window = Some(window_producer::start_window(title, width, height, background_color));
        self.camera = Some(camera_producer::start_camera(eye, at));
    }

    /// Open the window and render the frame many times
    fn render_image(&mut self, data: &Points) {
        // self.window.conrod_ui_mut().theme = gui::theme();
        // let ids = gui::Ids::new(self.window.conrod_ui_mut().widget_id_generator());
        // let mut app = gui::InfoBar::new_closed_state();

        self.require_window_not_null();
        self.require_camera_not_null();

        while self.render() {
            // self.render_frame(data, &ids, &mut app);
            self.render_frame(data);
        }
    }

    fn render_video(&mut self, ply_dir: PlyDir) -> Result<()> {
        video_renderer::render_video(self, ply_dir)
    }

    fn change_info_bar_state(&mut self) {

    }

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
    ) -> Result<()> {
        Ok(())
    }
}
