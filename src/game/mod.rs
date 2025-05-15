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

use entity::{actor::{Actor, ActorData}, entities::fairy::{self, Fairy}, player::Player, Actions, Entity, Priority};
use spaces::field::{self, Field};

use crate::utils::logger;

pub mod entity;
pub mod spaces;

//pub mod group;

pub struct Game {
	pub field:Field,
	pub player:Player,
	pub gamePieces: HashMap<String,GamePieces>, //at some point this could be a hash table for many players/angle entities
}

pub enum GamePieces {
	Fairy(Fairy)
}

impl Game {
	pub fn new() -> Self {


		Game {
			field: Field::new(),
			player: Player::new(),
			gamePieces: HashMap::new()
		}
	}

	pub fn init(&mut self, logger: &mut logger::Logger) {
		self.field.add_entity(self.player.player.clone());
		
		
		let mut fairy1:GamePieces = GamePieces::Fairy(Fairy::new(0,0, "Oolooroo".to_owned(), "0".to_owned()));

		let GamePieces::Fairy(ref mut fairy) = &mut fairy1;

		fairy.entity.actor.set_art_from_file("Fairy".to_owned());
		self.field.add_entity(fairy.entity.clone());
		self.gamePieces.insert(fairy.entity.id.clone(), fairy1);

		self.check_collision(&self.player.player);

		logger.log("Game initialized");
	}

	pub fn update(&mut self) {
		if let Some(GamePieces::Fairy(ref mut fairy)) = self.gamePieces.get_mut("0") {
			fairy.warp();
			self.field.set_entity(fairy.entity.clone());
		}
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