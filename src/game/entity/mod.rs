use std::fmt;

use actor::Actor;

pub mod player;
pub mod actor;
pub mod fairy;

pub mod being;

#[derive(Clone)]
pub struct Entity {
	pub x: i16,
	pub y: i16,
	pub priority: Priority,
	pub self_: char,
	pub id: String,
}

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum Priority {
	LOW = 0,
	MED = 1,
	HIG = 2,
}

impl Entity {

	pub fn new(x: i16, y: i16, self_: char, id:String, priority:Priority) -> Self {
		Entity { x, y, self_, id, priority}
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

impl fmt::Display for Entity {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} ({},{})", self.id, self.x, self.y)
	}
}


pub trait EntityData {
	fn get_health(self) -> i32;
	fn set_health(&mut self, health:i32);

	fn get_power(self) -> i32;
	fn set_power(&mut self, attack_power:i32);
}
