use angle_sc::Degrees;

use super::{Entity, Priority};

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

	pub fn is_facing(self, ents:Vec<Entity>) -> (bool, Option<Entity>) {
		let (player_x, player_y) = self.player.get_position();

		for e in ents {
			let (e_x, e_y) = e.get_position();
			if player_x-1 == e_x && self.direction == Direction_::LEFT {
				return (true, Some(e));
			}
			if player_x+1 == e_x && self.direction == Direction_::RIGHT {
				return (true, Some(e));
			}
			if player_y-1 == e_y && self.direction == Direction_::UP {
				return (true, Some(e));
			}
			if player_y+1 == e_y && self.direction == Direction_::DOWN {
				return (true, Some(e));
			}
		}
		(false, None)
	}
}
