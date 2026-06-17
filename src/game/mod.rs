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

use entity::{actor::ActorData, entities::fairy::{Fairy}, player::Player, Entity};
use haps_system::haps::{Event, Haps};
use spaces::field::Field;

use crate::utils::logger;

pub mod entity;
pub mod spaces;
pub mod haps_system;
pub mod vision;

//pub mod group;

pub struct Game {
	pub field:Field,
	pub player:Player,
	pub game_pieces: HashMap<String,GamePieces>, //at some point this could be a hash table for many players/angle entities
}

#[derive(Clone)]
pub enum GamePieces {
	Fairy(Fairy)
}

impl Game {
	pub fn new() -> Self {


		Game {
			field: Field::new(),
			player: Player::new(),
			game_pieces: HashMap::new()
		}
	}

	pub fn init(&mut self, logger: &mut logger::Logger) {
		self.field.add_entity(self.player.player.clone());

		// Add walls to create an interesting layout - starting close to player
		let walls = vec![
			// Wall directly ahead (x=2-4, y=3)
			(2, 3), (3, 3), (4, 3),
			// Vertical wall to the right
			(6, 1), (6, 2), (6, 3), (6, 4), (6, 5),
			// Horizontal wall below
			(3, 6), (4, 6), (5, 6), (6, 6), (7, 6),
			// Corner structure
			(10, 2), (10, 3), (10, 4), (11, 4), (12, 4),
		];

		let wall_count = walls.len();
		for (x, y) in walls {
			self.field.add_entity(Entity::new(x as i16, y as i16, '#', format!("wall_{}", x * 20 + y), entity::Priority::LOW));
		}

		logger.log(&format!("Game initialized with {} walls", wall_count));
	}

	pub fn update(&mut self, art: &mut String, prompt: &mut String, stats: &mut String, tick: usize, logger: &mut logger::Logger) {
		// Build this tick's event queue, then apply the events in priority order.
		let mut haps = Haps::new();

		if tick == 0 {
			haps.push(
				entity::Priority::HIG,
				Event::SpawnFairy {
					x: 10,
					y: 10,
					name: "Oolooroo".to_owned(),
					id: "0".to_owned(),
				},
			);
		}

		haps.push(entity::Priority::MED, Event::AdvanceWatchers);
		haps.push(entity::Priority::LOW, Event::ReapDead);

		for event in haps.drain_by_priority() {
			self.apply(event, logger);
		}

		// Art/prompt still flow out through parameters, so generation stays a
		// direct call rather than an event for now. Folding it in is the next
		// pass, once art/prompt live in game state instead of being threaded out.
		self.art_gen(art, prompt, stats);
	}

	/// Applies a single queued [`Event`]. This is the one place a tick's actions
	/// mutate the world, in the priority order `Haps` chose.
	fn apply(&mut self, event: Event, logger: &mut logger::Logger) {
		
		//what is the purpose of this. purpose not function in this case. as in match.
		//you have 6 minutes of my time how much is left over when I give it back? 
		//this should not be funny to you under any circumstances.

		match event {
			Event::SpawnFairy { x, y, name, id } => {
				self.create_entity(GamePieces::Fairy(Fairy::new(x, y, name, id)), logger);
			}

			Event::AdvanceWatchers => {
				for (_id, game_piece) in self.game_pieces.iter_mut() {
					match game_piece {
						GamePieces::Fairy(fairy) => {
							// Oolooroo is gentle for now: she stays put, a still
							// presence. (The stalking behavior is held for a darker
							// entity, when we are ready for it.)
							self.field.set_entity(fairy.entity.clone());
							logger.log(&format!("Oolooroo waits at ({}, {})", fairy.entity.x, fairy.entity.y));
						}
					}
				}
			}

			Event::ReapDead => {
				let mut dead: Vec<String> = Vec::new();
				for (_id, game_piece) in self.game_pieces.iter() {
					match game_piece {
						GamePieces::Fairy(fairy) => {
							if fairy.actor.health <= 0 {
								dead.push(fairy.entity.id.clone());
							}
						}
					}
				}
				for id in dead {
					self.field.remove_entity(id.clone());
					self.game_pieces.remove(&id);
				}
			}
		}
	}

	pub fn create_entity(&mut self, piece:GamePieces, logger: &mut logger::Logger) {
		let mut entity:GamePieces = piece;

		match &mut entity {
			GamePieces::Fairy(ref mut fairy) => {
				fairy.entity.set_position(fairy.entity.x, fairy.entity.y);
				if let Err(e) = fairy.actor.set_art_from_file("Fairy") {
					// A missing asset shouldn't crash a fresh clone: note it in
					// the log and show a visible placeholder instead.
					logger.log(&format!("Could not load Fairy art: {e}; using placeholder"));
					*fairy.actor.art_mut() = "[Fairy art missing]".to_string();
				}
				self.field.set_entity(fairy.entity.clone());
				self.game_pieces.insert(fairy.entity.id.clone(), entity);
			}
		}


	}

	pub fn art_gen(&mut self, art: &mut String, prompt: &mut String, stats: &mut String) {
		// An entity reveals itself when the player can actually *see* it: inside
		// the view cone, within range, and not hidden behind a wall. The first
		// one seen fills the portal with its art and line, and its stats fill the
		// Stats panel — gaze is the interaction that surfaces them.
		let facing = (self.player.heading.0 as f32 - 90.0) * std::f32::consts::PI / 180.0;
		let player_pos = self.player.player.get_position();
		let player_id = self.player.player.id.clone();

		for (_id, piece) in self.game_pieces.iter() {
			match piece {
				GamePieces::Fairy(fairy) => {
					let target = fairy.entity.get_position();
					let target_id = fairy.entity.id.clone();
					let seen = vision::can_see(
						player_pos,
						facing,
						std::f32::consts::PI / 2.0,
						20.0,
						target,
						|x, y| match self.field.get_entity_by_position(x, y) {
							Some(e) => e.id != player_id && e.id != target_id,
							None => false,
						},
					);
					if seen {
						*art = fairy.actor.art.clone();
						*prompt = fairy.actor.prompt.clone();
						let (name, health, power) = fairy.actor.get_stats();
						*stats = format!("{name}\n\nHP   {health}\nATK  {power}");
						return;
					}
				}
			}
		}
	}


	pub fn check_near(&self, entity: &Entity, entities: &mut Vec<Entity>) {
		// Check for collision with other entities
		// Return true if collision occurs, false otherwise
		let (x, y) = entity.get_position();

		let near_mask = [
			(x.saturating_sub(1), y), // left
			(x + 1, y),               // right
			(x, y.saturating_sub(1)), // up
			(x, y + 1),               // down
		];

		for (nx, ny) in near_mask {
			if let Some(e) = self.field.get_entity_by_position(nx, ny) {
				if e.id != entity.id {
					entities.push(e.clone());
				}
			} else {
				entities.push(entity.clone());
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::game::entity::entities::fairy::Fairy;
	use angle_sc::Degrees;

	fn fairy_at(x: i16, y: i16, art: &str, id: &str) -> Fairy {
		let mut f = Fairy::new(x, y, "Test".to_owned(), id.to_owned());
		f.actor.art = art.to_owned();
		f
	}

	#[test]
	fn reveals_entity_in_view() {
		let mut game = Game::new();
		game.player.player.set_position(0, 0);
		game.player.heading = Degrees(90.0); // facing RIGHT (+x), straight at the fairy

		let fairy = fairy_at(5, 0, "the-fairy", "f1");
		game.field.set_entity(fairy.entity.clone());
		game.game_pieces.insert("f1".to_owned(), GamePieces::Fairy(fairy));

		let mut art = String::new();
		let mut prompt = String::new();
		let mut stats = String::new();
		game.art_gen(&mut art, &mut prompt, &mut stats);

		assert_eq!(art, "the-fairy", "a clear line of sight should reveal the entity");
		assert!(stats.contains("Test"), "the seen entity's stats should fill the panel");
	}

	#[test]
	fn hides_entity_out_of_view() {
		let mut game = Game::new();
		game.player.player.set_position(0, 0);
		game.player.heading = Degrees(0.0); // facing UP (-y); the fairy is off to +x

		let fairy = fairy_at(5, 0, "the-fairy", "f1");
		game.field.set_entity(fairy.entity.clone());
		game.game_pieces.insert("f1".to_owned(), GamePieces::Fairy(fairy));

		let mut art = String::new();
		let mut prompt = String::new();
		let mut stats = String::new();
		game.art_gen(&mut art, &mut prompt, &mut stats);

		assert!(art.is_empty(), "an entity outside the view cone should stay hidden");
		assert!(stats.is_empty(), "no gaze, no stats readout");
	}
}
