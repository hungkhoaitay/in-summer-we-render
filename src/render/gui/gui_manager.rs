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

use crate::gui::gui::GUI;

use kiss3d::conrod::widget::id::Generator;

pub struct GUIManager {
    widget_id_generator: Option<Generator>
}

impl GUIManager {
    pub fn new() -> Box<dyn GUI> where Self: Sized {
        Box::new(UIManager {
        })
    }
}


impl GUI for GUIManager {
    fn start(widget_id_generator: Generator) {

    };

    fn render_gui();
}
