use crate::errors::*;
use crate::ply::Ply;
use crate::ply_dir::PlyDir;
use crate::points::Points;
use nalgebra::Point3;

use crate::gui::ids::Ids;
use kiss3d::conrod::widget::id::Generator;

pub trait GUI {
    fn start(widget_id_generator: Generator);

    fn render_gui();
}
