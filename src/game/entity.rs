pub struct Entity {
	pub x: usize,
	pub y: usize,
	pub self_: char,
}

impl Entity {
	
	pub fn new(x: usize, y: usize, self_: char) -> Self {
		Entity { x, y, self_ }
	}

	pub fn update(mut self, e:Entity){
		self.x = e.x;
		self.y = e.y;
		self.self_ = e.self_;
	}

	pub fn move_up(&mut self){
		let (x, mut y) = self.get_position();
		y -= 1;
		self.set_position(x, y);
	}
	pub fn move_down(&mut self){
		let (x, mut y) = self.get_position();
		y += 1;
		self.set_position(x, y);
	}
	pub fn move_left(&mut self){
		let (mut x, y) = self.get_position();
		x -= 1;
		self.set_position(x, y);
	}
	pub fn move_right(&mut self){
		let (mut x, y) = self.get_position();
		x += 1;
		self.set_position(x, y);

	}

	fn set_position(&mut self, new_x: usize, new_y: usize) {
		self.x = new_x;
		self.y = new_y;
	}
	
	fn get_position(&self) -> (usize, usize) {
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