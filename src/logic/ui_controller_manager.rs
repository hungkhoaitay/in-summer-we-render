use crate::logic::ui_controller::UIController;
use crate::render::{ui::UI, ui_manager::UIManager};
use kiss3d::conrod::event::Event;

pub struct UIControllerManager {}

impl UIControllerManager {
    pub fn new() -> Box<dyn UIController>
    where
        Self: Sized,
    {
        Box::new(UIControllerManager {})
    }
}

impl UIController for UIControllerManager {
    fn handle_events(self: &Self, event: Event) {}
}
