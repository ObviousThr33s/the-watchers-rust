/*pub mod trading;
pub mod crafting; //includes resource gathering

pub mod dialogue;
pub mod skills;
pub mod combat;

pub mod items;
pub mod levels;

pub mod entities;
pub mod world;*/

use entity::{actor::Actor, player::Player, Entity, Priority};
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
		
		let mut actor2 = Actor::new( 
					"Death".to_owned(), 
					1, 1);
			actor2.set_art_from_file();

		let mut actor1 = Actor::new( 
					"Fairy".to_owned(), 
					1, 1);
			actor1.set_art_from_file();

		let fae = Entity::new(1, 1, 'F', "Fae".to_owned(), Priority::LOW, actor1.clone());
		let faed = Entity::new(1, 1, 'G', "Faed".to_owned(), Priority::HIG, actor1);

		let death = Entity::new(2,0,'D', "Death".to_owned(), Priority::LOW, actor2);

		self.field.add_entity(faed);
		self.field.add_entity(fae);
		self.field.add_entity(death);

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


		let i = 0;
		for (nx, ny) in near_mask {
			if let Some(e) = self.field.get_entity_by_position(nx, ny) {
				if e.get() != entity.get() {
					near_entities.push(e.clone());
				}
			}else{
				near_entities.push(entity.clone());
			}
		}

		for n in near_entities.clone() {
			logger::Logger::save_log_sp("res/logs", "near.txt", format!("{}", n.to_string()));
		}

		near_entities
	}
}