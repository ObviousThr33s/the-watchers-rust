//! The player — the lens the world is seen through, and the one place the game
//! hardcodes a spawn (everything else is data loaded from `.being` files). Unlike
//! a being, the player is never concealed; the map and the first-person view both
//! read straight off the position and facing kept here.

use angle_sc::Degrees;

use super::{Entity, Priority, PLAYER};

/// The player: the in-field [`Entity`] (glyph + position) paired with a
/// continuous `heading` in degrees and the cardinal `direction` snapped from it.
#[derive(Clone)]
pub struct Player {
	pub player:Entity,
	pub heading:Degrees,
	pub direction:Direction_

}

/// The four cardinal facings, snapped from the player's `heading`. (The trailing
/// `_` keeps it clear of any other `Direction` that might come into scope.)
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

	/// Spawn the player at `(2, 2)` facing up — the alcove [`Game::init`](crate::game::Game::init)
	/// builds its walls around.
	pub fn new() -> Self {
		Self {
			player: Entity::new(
				2,
				2,
				'^',
				PLAYER,
				Priority::MED,
			),
			heading: angle_sc::Degrees(0.0),
			direction:Direction_::UP
		}
	}

	/// Turn by subtracting `amnt` degrees from the heading, then re-snap the
	/// cardinal direction and glyph (see [`poll_heading`](Self::poll_heading)).
	pub fn add_direction(&mut self, amnt:f64){
		let old_dir = self.heading.0;
		self.heading = Degrees(old_dir) - Degrees(amnt);
		self.poll_heading();
	}

	/// Turn the other way — adds `amnt` degrees to the heading — then re-snaps.
	pub fn sub_direction(&mut self, amnt:f64){
		let old_dir = self.heading.0;
		self.heading = Degrees(old_dir) + Degrees(amnt);
		self.poll_heading();
	}


	/// Snap `heading` to the nearest cardinal [`Direction_`] and update the glyph
	/// (`^ v < >`) to match. Only the exact angles (0 / ±90 / 180) are handled today.
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
