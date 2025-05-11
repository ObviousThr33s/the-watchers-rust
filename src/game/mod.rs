/*pub mod trading;
pub mod crafting; //includes resource gathering

pub mod dialogue;
pub mod skills;
pub mod combat;

pub mod items;
pub mod levels;

pub mod entities;
pub mod world;*/

use entity::{player::Player, Entity, Priority};
use spaces::field::Field;

use crate::utils::logger;

pub mod entity;
pub mod spaces;

//pub mod group;

pub struct Game {
	pub field:Field,
	pub player:Player, //at some point this could be a hash table for many players/angle entities
}

impl Game {
	pub fn new() -> Self {


		Game {
			field: Field::new(),
			player: Player::new(),
		}
	}

	pub fn init(&mut self, logger: &mut logger::Logger) {
		self.field.add_entity(self.player.player.clone());
		
		let static_entity = Entity::new(1, 1, 'F', "Fae".to_owned(), Priority::LOW);
		self.field.add_entity(static_entity);
		self.check_collision(&self.player.player);

		logger.log("Game initialized");
	}

	pub fn check_collision(&self, entity: &Entity) -> Vec<Entity> {
		// Check for collision with other entities
		// Return true if collision occurs, false otherwise

		let mut near_entities:Vec<Entity> = Vec::new();

		let (x, y) = entity.get_position();
		
		let near_mask = vec![
			(x - 1, y), // left
			(x + 1, y), // right
			(x, y - 1), // up
			(x, y + 1), // down
		];

		for (nx, ny) in near_mask {
			if let Some(e) = self.field.get_entity_by_position(nx, ny) {
				if e.get() != entity.get() {
					near_entities.push(e.clone());
				}
			}
		}

		for n in near_entities.clone() {
			logger::Logger::save_log_sp("res/logs", "near.txt", format!("{}", n.to_string()));
		}

		near_entities
	}
}