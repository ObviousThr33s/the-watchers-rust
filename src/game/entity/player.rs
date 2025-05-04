use angle_sc::Degrees;

use super::{Entity, Priority};

pub struct Player {
	pub player:Entity,
	pub heading:Degrees,
	pub direction:Direction_
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Direction_ {
	NORTH,
	SOUTH,
	EAST,
	WEST
}

impl Clone for Player {
	fn clone(&self) -> Self {
		Self { 
			player: self.player.clone(),
			heading:Degrees(0.0f64).clone(),
			direction:self.direction.clone()
		}
	}
}

impl Player {

	pub fn new() -> Self {
		Self {
			player:Entity {
				x: 2,
				y: 2,
				priority: Priority::MED,
				self_: '^', 
				id: "Player".to_owned() 
			},
			heading: angle_sc::Degrees(0.0),
			direction:Direction_::NORTH
		}
	}

	pub fn add_direction(&mut self, amnt:f64){
		let old_dir = self.heading.0;
		self.heading = Degrees(old_dir) - Degrees(amnt);
		self.poll_heading();
	}

	pub fn sub_direction(&mut self, amnt:f64){
		let old_dir = self.heading.0;
		self.heading = Degrees(old_dir) + Degrees(amnt);
		self.poll_heading();
	}
	

	pub fn poll_heading(&mut self) {

		if self.heading.0 == -90.0 {
			self.direction = Direction_::EAST;
			self.player.self_ = '<';
		}
		if self.heading.0 == 90.0 {
			self.direction = Direction_::WEST;
			self.player.self_ = '>';
		}
		if self.heading.0 == 180.0 {
			self.direction = Direction_::SOUTH;
			self.player.self_ = 'v';
		}
		if self.heading.0 == 0.0 {
			self.direction = Direction_::NORTH;
			self.player.self_ = '^';
		}

	}
}
