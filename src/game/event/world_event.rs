pub struct WorldEvent {
	pub id: u32,
	pub description: String,
	pub timestamp: u64,
}

impl Clone for WorldEvent {
	fn clone(&self) -> Self {
		Self { id: self.id.clone(), description: self.description.clone(), timestamp: self.timestamp.clone() }
	}
}

impl WorldEvent {
	pub fn new(id: u32, description: &str, timestamp: u64) -> Self {
		WorldEvent {
			id,
			description: description.to_string(),
			timestamp,
		}
	}
}






