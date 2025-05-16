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
use spaces::field::Field;

use crate::utils::logger;

pub mod entity;
pub mod spaces;

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
		
		self.create_entity(GamePieces::Fairy(Fairy::new(0,0, "Oolooroo".to_owned(), "0".to_owned())));
		
		self.check_collision(&self.player.player);

		logger.log("Game initialized");
	}

	pub fn update(&mut self, art: &mut String, prompt: &mut String, tick: usize) {
		
		let events = vec![
			{
				self.entity_gen(art, prompt);

				for (_id, game_piece) in self.game_pieces.iter_mut() {
					match game_piece {
						GamePieces::Fairy(ref mut fairy) => {
							fairy.warp(tick);
							self.field.set_entity(fairy.entity.clone());
						}
					}
				}
				if tick % 10 == 0 {
					self.create_entity(GamePieces::Fairy(Fairy::new(0,0, "Ooloorootoo".to_owned(), tick.to_string())));
				}
			}
		];

		for e in events {
			e
		}
	}

	pub fn create_entity(&mut self, piece:GamePieces) {
		let mut entity:GamePieces = piece;

		match &mut entity {
			GamePieces::Fairy(ref mut fairy) => {
				fairy.entity.set_position(0, 0);
				fairy.actor.set_art_from_file("Fairy".to_owned());
				self.field.set_entity(fairy.entity.clone());
				self.game_pieces.insert(fairy.entity.id.clone(), entity);
			}
		}

		
	}

	pub fn entity_gen(&mut self, art: &mut String, prompt: &mut String) {

		if let Some(GamePieces::Fairy(ref mut fairy)) = self.game_pieces.get_mut("0") {
			//fairy.warp();
			self.field.set_entity(fairy.entity.clone());
		}


		let near = self.check_collision(&self.player.player);

		let (is_facing, e) = self.player.clone().is_facing(near);
		
		if is_facing && e.is_some(){
			if let Some(GamePieces::Fairy(ref mut fairy)) = self.game_pieces.get_mut(&e.unwrap().id) {
				(*art,*prompt) = (fairy.actor.art.clone(), fairy.actor.prompt.clone());
			}
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


		for (nx, ny) in near_mask {
			if let Some(e) = self.field.get_entity_by_position(nx, ny) {
				if e.get() != entity.get() {
					near_entities.push(e.clone());
				}
			}else{
				near_entities.push(entity.clone());
			}
		}

		near_entities
	}
}