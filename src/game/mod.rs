//! The game world and its clock. [`Game`] owns the [`Field`] every entity stands
//! in and the [`Haps`] event queue that paces a tick. `init` populates the world
//! (the player, a starter alcove, a sown forest); `update` is where per-tick
//! behavior will land as the rebuild continues. Pure engine — everything it
//! places (beings, flora) is data loaded from `.being` files, never hardcoded.

/*pub mod trading;
pub mod crafting; //includes resource gathering

pub mod dialogue;
pub mod skills;
pub mod combat;

pub mod items;
pub mod levels;

pub mod entities;
pub mod world;*/

use spaces::field::Field;
use spaces::terrain::{self, Sowing};

use crate::{utils::logger};

pub mod entity;
pub mod spaces;
pub mod vision;
pub mod recollection;
pub mod haps;

//pub mod group;

/// The world: the [`Field`] of everything placed in it, plus the per-tick event
/// queue ([`Haps`], named `time` for the tempo it keeps).
pub struct Game {
	pub field:Field,
}

impl Game {
	/// An empty world — a bare field and an empty event queue. Call [`init`](Self::init)
	/// (or the first [`update`](Self::update)) to populate it.
	pub fn new() -> Self {
		Game {
			field: Field::new(),
		}
	}

	/// Populate the world for the first time — place the player, raise the starter
	/// alcove, and sow the forest from the flora `.being` files. Meant to run once,
	/// at tick 0 (see [`update`](Self::update)).
	pub fn init(&mut self, logger: &mut logger::Logger) {
		// The player is the one hardcoded spawn (see entity::player) — the lens
		// the world is seen through, not data like the beings. Place them in the
		// field so the renderer always has, at minimum, the player to draw. The
		// rest of the population (beings, behaviors) is the rebuild still to come.
		self.field.add_entity(entity::player::Player::new().player);

		// Structural geometry (not narrative): a small alcove around the player
		// so the first-person view has walls to cast against. The player faces
		// "up" (-y) from (2,2), so the front wall sits at y=0 dead ahead with a
		// wall down each side. Scaffolding for the view — this will likely move to
		// per-field level data later, not stay hardcoded here.
		let walls = [
			(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), // front wall, dead ahead
			(0, 1), (0, 2),                         // left wall
			(4, 1), (4, 2),                         // right wall
		];
		for (x, y) in walls {
			self.field.add_entity(entity::Entity::new(
				x, y, '#', format!("wall_{x}_{y}"), entity::Priority::LOW,
			));
		}

		logger.log(&format!("Player placed; {} walls built", walls.len()));

		// Sow the field with flora — trees, brush, and brambles loaded from their
		// `.being` files. The story (glyph, art, the line each surfaces when seen)
		// lives in those files; this only scatters them into groves. They're solid,
		// so the forest is at once scenery on the Map and walls the view casts
		// against. The clearing keeps the spawn alcove open. (The hardcoded alcove
		// above can retire once the forest alone gives the view enough to cast on.)
		let flora = terrain::load_flora();
		let planted = terrain::sow(&mut self.field, &flora, Sowing {
			x0: 0,
			y0: 0,
			width: 48,
			height: 30,
			clear_around: (2, 2),
			clearing: 3,
			threshold: 0.35,
			scale: 0.18,
			seed: 1,
		});
		logger.log(&format!("Sowed {planted} flora from {} kinds", flora.len()));
	}

	/// Advance the world by one tick. At tick 0 it bootstraps via [`init`](Self::init);
	/// the per-tick event/behavior pipeline (drain [`Haps`] in arrival order, run
	/// gaze-gated behavior) is the rebuild still ahead, so for now this only seeds
	/// the world. `recollection` and `logger` are threaded in ahead of that wiring.
	pub fn update(&mut self, tick: usize, logger: &mut logger::Logger) {
		// Build this tick's event queue, then apply the events in arrival order.
		
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

		let mut lamp = Render::init(20, 20);
		lamp.rasterize(&game.field);

		assert!(
			lamp.to_string().contains(player.self_),
			"the renderer should draw the player glyph, not an empty panel"
		);
	}

	/// The first-person view needs walls to cast against; an empty field renders
	/// as blank space. init must build the alcove so the live viewport shows a
	/// surface dead ahead of the (up-facing) player.
	#[test]
	fn init_builds_a_room_so_the_first_person_view_has_walls() {
		let mut logger = Logger::new(Time::new(), "test".to_owned());
		let mut game = Game::new();
		game.init(&mut logger);

		assert!(
			game.field.get_entity_by_position(2, 0).is_some(),
			"expected a wall dead ahead of the up-facing player at (2,0)"
		);
	}
}
