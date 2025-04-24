use crate::utils::logger::Logger;

pub struct Space {
	pub name: String,
	pub description: String,
	pub id: u32,
	pub x: i32,
	pub y: i32,
	pub z: i32,
	pub width: u16,
	pub height: u16,
	pub logger: Logger
}

impl Space {
	pub fn new(name: &str, description: &str, id: u32, x: i32, y: i32, z: i32, width: u16, height: u16) -> Self {
		Self {
			name: name.to_string(),
			description: description.to_string(),
			id,
			x,
			y,
			z,
			width,
			height,
			logger: Logger::new()
		}
	}

	pub fn log(&mut self, message: &str) {
		self.logger.log(message);
	}
}