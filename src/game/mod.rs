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
	
		//self.field.add_entity(self.player.clone());

		// Add walls to create an interesting layout - starting close to player

		//logger.log(&format!("Game initialized with {} walls", wall_count));
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
