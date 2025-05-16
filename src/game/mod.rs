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
use haps_system::haps::Haps;
use spaces::field::Field;

use crate::utils::logger;

pub mod entity;
pub mod spaces;
pub mod haps_system;

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
		
		logger.log("Game initialized");
	}

	pub fn update(&mut self, art: &mut String, prompt: &mut String, tick: usize, logger: &mut logger::Logger) {
		
		let mut haps:Haps = Haps::new();

		haps.add_event(
		if tick == 3 {
				self.create_entity(
					GamePieces::Fairy(
						Fairy::new(
							0,0,
							"Oolooroo".to_owned(),
							"0".to_owned()
						)
					)
				)
		}
		);

		haps.add_event(self.art_gen(art, prompt));

		haps.add_event(
			for (_id, game_piece) in self.game_pieces.iter_mut() {
				match game_piece {
					GamePieces::Fairy(ref mut fairy) => {
					fairy.warp(tick);
					
					self.field.set_entity(fairy.entity.clone());
					logger.log(&format!("Fairy {} health: {}", fairy.entity.id, fairy.actor.health));

				}
			}
		});
	
		let mut to_remove: Vec<String> = Vec::new();
		
		haps.add_event(
			for(_id, game_piece) in self.game_pieces.iter_mut() {
				match game_piece {
					GamePieces::Fairy(ref mut fairy) => {
						if fairy.actor.health <= 0 {
							// Mark the entity for removal
							to_remove.push(fairy.entity.id.clone());
							// Remove from field immediately
							self.field.remove_entity(fairy.entity.id.clone());
						}
					}
				}
			}
		);
		
		// Now remove the marked entities from game_pieces
		haps.add_event(
			for id in &to_remove {
				self.game_pieces.remove(id);
			}
		);

		

		haps.execute();
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

	pub fn art_gen(&mut self, art: &mut String, prompt: &mut String) {

		let mut near:Vec<Entity> = Vec::new();

		self.check_near(&self.player.player, &mut near);

		let (is_facing, e) = self.player.clone().is_facing(near);
		
		if is_facing && e.is_some(){
			if let Some(GamePieces::Fairy(ref mut fairy)) = self.game_pieces.get_mut(&e.unwrap().id) {
				(*art,*prompt) = (fairy.actor.art.clone(), fairy.actor.prompt.clone());
			}
		}

	}



	pub fn check_near(&self, entity: &Entity, entities:&mut Vec<Entity>) {
		// Check for collision with other entities
		// Return true if collision occurs, false otherwise
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
					entities.push(e.clone());
				}
			}else{
				entities.push(entity.clone());
			}
		}
	}
}