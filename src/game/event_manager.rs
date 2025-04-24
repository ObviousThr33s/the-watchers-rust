use super::world_event::WorldEvent;

pub struct EventManager {
	events: Vec<WorldEvent>,
}
impl Clone for EventManager {
	fn clone(&self) -> Self {
		Self { events: self.events.clone() }
	}
}

impl EventManager {
	pub fn new() -> Self {
		EventManager {
			events: Vec::new(),
		}
	}

	// Add a new event
	pub fn add_event(&mut self, event: WorldEvent) {
		self.events.push(event);
	}

	// Get the next event (FIFO)
	pub fn get_event(&mut self) {
		
	}

	// Peek at the next event without removing it
	pub fn peek_event(&self) {
	}

	// Check if there are any events
	pub fn has_events(&self) -> bool {
		!self.events.is_empty()
	}
}