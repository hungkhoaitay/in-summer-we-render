use kiss3d::conrod::event::Event;

pub trait UIController {
    fn handle_events(self: &Self, event: Event);
}
