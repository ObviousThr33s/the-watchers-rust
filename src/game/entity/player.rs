use angle_sc::Degrees;

use super::{Entity, Priority};

#[derive(Clone)]
pub struct Player {
	pub player:Entity,
	pub heading:Degrees,
	pub direction:Direction_

}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Direction_ {
	UP = 0,
	DOWN = 1,
	RIGHT = 2,
	LEFT = 3,
}

//this may be the only place we hardcode anything. but even then-- controller support.
//maybe learn how the player plays.

impl Player {

	pub fn new() -> Self {
		Self {
			player: Entity::new(
				2,
				2,
				'^', 
				"Player".to_owned(),
				Priority::MED,
			),
			heading: angle_sc::Degrees(0.0),
			direction:Direction_::UP
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
			self.direction = Direction_::LEFT;
			self.player.self_ = '<';
		}
		if self.heading.0 == 90.0 {
			self.direction = Direction_::RIGHT;
			self.player.self_ = '>';
		}
		if self.heading.0 == 180.0 {
			self.direction = Direction_::DOWN;
			self.player.self_ = 'v';
		}
		if self.heading.0 == 0.0 {
			self.direction = Direction_::UP;
			self.player.self_ = '^';
		}

	}
}
