use crate::points::Points;
use crate::render::ui_manager::UIManager;

pub(crate) fn render_frame(ui_manager: &mut UIManager, data: &Points) {
    for point in &data.data {
        ui_manager.draw_point_with_size(
            &point.get_coord().get_point3(),
            &point.get_color().get_point3(),
            point.point_size,
        );
    }

    // gui(ids, app, uiManager);
}
