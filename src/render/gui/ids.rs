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

pub static INFO_BUTTON_LABEL: &str = "info";

widget_ids! {
    pub struct Ids {
        canvas,
        toggle,
        text_info,
    }
}
