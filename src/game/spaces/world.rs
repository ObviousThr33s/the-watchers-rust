
use crate::game::event::event_manager::EventManager;
use crate::utils::logger::Logger;

use crate::game::event::world_event;
use crate::utils::time::Time;

use super::space::Space;
use crate::game::transforms::*;
use crate::game::transforms::terrain::{Terrain, TerrainTypes};

pub struct World {
	event_q:EventManager,
}

impl Clone for World{
	fn clone(&self) -> Self {
		Self { 
			event_q: self.event_q.clone(), 
		}
	}
}

impl Space for World {
	fn new() -> Self {
		World{
			event_q: EventManager::new(),
		}
	}

	fn init(self, logger_:&mut Logger) -> World{
		logger_.log("World initialization started...");




		logger_.log("World initialization done.");
		
		self
	}
	fn update(self, _tick:i64) -> Self{
		self
	}
	fn save(self){
	}
}