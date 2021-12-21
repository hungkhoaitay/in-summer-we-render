use kiss3d::conrod;
use kiss3d::conrod::event::{Event, Input};
use kiss3d::conrod::input::{Button, Key};
use kiss3d::conrod::position::Positionable;
use kiss3d::conrod::widget_ids;

use crate::gui::gui_states::{close_state, open_state, State};

pub const WIN_W: u32 = 600;
pub const WIN_H: u32 = 420;

const MARGIN: conrod::Scalar = 10.0;
const INFO_BUTTON_SIZE: conrod::Scalar = 40.0;
const TEXT_INFO_HEIGHT: conrod::Scalar = 100.0;

pub fn theme() -> conrod::Theme {
    use conrod::position::{Align, Direction, Padding, Position, Relative};
    conrod::Theme {
        name: "Demo Theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod::color::DARK_CHARCOAL,
        shape_color: conrod::color::LIGHT_CHARCOAL,
        border_color: conrod::color::BLACK,
        border_width: 0.0,
        label_color: conrod::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 14,
        font_size_small: 12,
        widget_styling: conrod::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}
