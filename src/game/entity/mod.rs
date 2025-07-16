use actor::Actor;

pub mod player;
pub mod floor;
pub mod actor;

pub mod entities;

pub enum GameObject {
	Player(Entity),
	Floor(Entity),
	Fairy(Entity),
}

pub struct Entity {
	pub x: i16,
	pub y: i16,
	pub priority: Priority,
	pub self_: char,
	pub id: String,
}

#[derive(Hash, Eq, PartialEq, PartialOrd, Clone)]
pub enum Priority {
	LOW = 0,
	MED = 1,
	HIG = 2,
}

impl Entity {

	pub fn new(x: i16, y: i16, self_: char, id:String, priority:Priority) -> Self {
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
	pub fn set_position(&mut self, new_x: i16, new_y: i16) {
		self.x = new_x;
		self.y = new_y;
	}
	
	#[inline]
	pub fn get(&self) -> (i16, i16, &str) {
		(self.x, self.y, &self.id)
	}

	#[inline]
	pub fn get_position(&self) -> (i16, i16) {
		(self.x, self.y)
	}

}

impl Clone for Entity {
	fn clone(&self) -> Self {
		Entity {
			x: self.x,
			y: self.y,
			self_: self.self_,
			priority: self.priority.clone(),
			id: self.id.clone(),
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


pub trait EntityData {
	fn get_health(self) -> i32;
	fn set_health(&mut self, health:i32);

	fn get_power(self) -> i32;
	fn set_power(&mut self, attack_power:i32);
}

pub trait Actions {
	fn attack(self, actor: &mut Actor);
}