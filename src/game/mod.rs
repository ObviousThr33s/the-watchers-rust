/*pub mod trading;
pub mod crafting; //includes resource gathering

pub mod dialogue;
pub mod skills;
pub mod combat;

pub mod items;
pub mod levels;

pub mod entities;
pub mod world;*/

use std::collections::HashMap;
//we have to write this before we write the other stuff but we have to write the other stuff before we write this. 

use spaces::field::Field;

use crate::{game::{haps::Haps, recollection::Recollection}, utils::{logger, time}};

pub mod entity;
pub mod spaces;
pub mod vision;
pub mod recollection;
pub mod haps;

//pub mod group;

pub struct Game {
	pub field:Field,
	time:Haps,
}

impl Game {
	pub fn new() -> Self {


		Game {
			field: Field::new(),
			time: Haps::new(),
		}
	}

	pub fn init(&mut self, logger: &mut logger::Logger) {
		// The player is the one hardcoded spawn (see entity::player) — the lens
		// the world is seen through, not data like the beings. Place them in the
		// field so the renderer always has, at minimum, the player to draw. The
		// rest of the population (beings, behaviors) is the rebuild still to come.
		self.field.add_entity(entity::player::Player::new().player);
		logger.log("Player placed in the field");
	}

	pub fn update(&mut self, tick: usize, logger: &mut logger::Logger, recollection: Recollection) {
		// Build this tick's event queue, then apply the events in priority order.
		
		if tick == 0 {
			self.init(logger);
		}

		
		// Art/prompt still flow out through parameters, so generation stays a
		// direct call rather than an event for now. Folding it in is the next
		// pass, once art/prompt live in game state instead of being threaded out.
	}

}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::gfx::charset::CHARSETS;
	use crate::gfx::minimap::render::Render;
	use crate::utils::{logger::Logger, time::Time};

	/// The renderer drew nothing because the field was empty after the refactor.
	/// init must put the player in the world — the floor the rest of the rebuild
	/// (beings, behaviors, gaze) stands on.
	#[test]
	fn init_places_the_player_in_the_field() {
		let mut logger = Logger::new(Time::new(), "test".to_owned());
		let mut game = Game::new();
		game.init(&mut logger);

		assert!(
			game.field.get_entity_by_id("Player").is_some(),
			"init must place the player in the field"
		);
	}

	/// The render pipeline itself is sound (see gfx::minimap::render tests); the
	/// bug was an empty field, so it painted nothing. With the player placed,
	/// rasterizing the field must show the player's glyph — content, not a blank
	/// panel. This is the regression guard for "the renderer isn't working."
	#[test]
	fn renderer_draws_what_init_populates() {
		let mut logger = Logger::new(Time::new(), "test".to_owned());
		let mut game = Game::new();
		game.init(&mut logger);

		let player = game
			.field
			.get_entity_by_id("Player")
			.expect("init must place the player")
			.clone();

		let mut lamp = Render::init(20, 20, CHARSETS::Charset0);
		lamp.rasterize(&game.field);

		assert!(
			lamp.to_string().contains(player.self_),
			"the renderer should draw the player glyph, not an empty panel"
		);
	}
}
