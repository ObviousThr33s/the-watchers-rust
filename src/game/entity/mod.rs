pub mod player;
pub mod floor;
pub mod wall_type;

pub struct Entity {
	pub x: usize,
	pub y: usize,

	pub priority:Priority,

	pub self_: char,
	pub id:String
}

#[derive(PartialEq, PartialOrd, Clone)]
pub enum Priority {
	LOW = 0,
	MED = 1,
	HIG = 2,
}

impl Entity {
	
	pub fn new(x: usize, y: usize, self_: char, id:String, priority:Priority) -> Self {
		Entity { x, y, self_, id, priority}
	}

	pub fn update(&mut self, e: Entity) {
		self.x = e.x;
		self.y = e.y;
		self.self_ = e.self_;
		self.id = e.id.clone();
	}

	pub fn move_up(&mut self){
		let (x, mut y) = self.get_position();
		if y != 0 {
			y -= 1;
		}

		self.set_position(x, y);
	}
	pub fn move_down(&mut self){
		let (x, mut y) = self.get_position();
		y += 1;
		self.set_position(x, y);
	}
	pub fn move_left(&mut self){
		let (mut x, y) = self.get_position();
		if x != 0 {
			x -= 1;
		}
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
	
	pub fn get(&self) -> (usize, usize, String){
		(self.x, self.y, self.id.clone())
	}

	pub fn get_position(&self) -> (usize, usize) {
		(self.x, self.y)
	}

}

impl Clone for Entity {
	fn clone(&self) -> Self {
		Entity {
			x: self.x.clone(),
			y: self.y.clone(),
			self_: self.self_.clone(),
			priority: self.priority.clone(),
			id:self.id.clone()
		}
	}
}

impl ToString for Entity {
	fn to_string(&self) -> String {
		let e_y = self.x.clone();
		let e_x = self.y.clone();
		let e_id = self.id.clone();
		let s:String = format!("{} ({},{})",e_id, e_x, e_y).clone();
		let s_ = s.clone();
		s_
	}
}