use std::thread::Thread;

use rand::{rngs::ThreadRng, Rng};

use crate::{game::entity::Entity, utils::logger::{self, Logger}};


pub(crate) struct MainLoop {
	tick:usize,
}

impl MainLoop {


	pub fn new() -> MainLoop {
		MainLoop { 
			tick: 0, 
		}
	}

	#[allow(unused)]
	pub async fn main_loop(&mut self, mut logger:&mut Logger, mut entities:Vec<Entity>, tick:usize, mut r:ThreadRng) -> (Vec<Entity>, usize){

		if entities.len() < 2 {
			let mut new_entities:Vec<Entity> = vec![Entity {
				x:r.random_range(0..10),
				y:r.random_range(0..10),
				self_:'E',
			}];	

			entities.append(&mut new_entities);
			logger.
			log("Drew entity");
		}

		Self::place_entity(&mut entities[1], r.random_range(0..10), r.random_range(0..10));

		logger.log("Main Loop finished");

		(entities, self.tick)
	}

	pub fn place_entity(e:&mut Entity, x:usize, y:usize){
		e.x = x;
		e.y = y;
	}
}
