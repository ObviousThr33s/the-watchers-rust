use std::clone;

pub struct Entity {
	pub x: usize,
	pub y: usize,
	pub self_: char,
}

impl Entity {
	
	pub fn new(x: usize, y: usize, self_: char) -> Self {
		Entity { x, y, self_ }
	}

	pub fn set_position(&mut self, new_x: usize, new_y: usize) {
		self.x = new_x;
		self.y = new_y;
	}
	
	pub fn get_position(&self) -> (usize, usize) {
		(self.x, self.y)
	}
}

impl Clone for Entity {
	fn clone(&self) -> Self {
		Entity {
			x: self.x,
			y: self.y,
			self_: self.self_,
		}
	}
}