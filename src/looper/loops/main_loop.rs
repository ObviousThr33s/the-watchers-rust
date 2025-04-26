
use rand::rngs::ThreadRng;

use crate::{game::entity::Entity, utils::logger::Logger};


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

		logger.log("Main loop ending...");

		(entities, self.tick)
	}

	pub fn place_entity(e:&mut Entity, x:usize, y:usize){
		e.x = x;
		e.y = y;
	}
}
