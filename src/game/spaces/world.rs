
use crate::utils::logger::Logger;

use crate::game::world_generation::world_event::EventManager;
use crate::utils::time::Time;
use crate::game::world_generation::terrain::{Terrain, TerrainTypes};

pub struct World {
	l:Logger,
	e_q:EventManager,
}

impl Clone for World{
	fn clone(&self) -> Self {
		Self { l: self.l.clone(), e_q: self.e_q.clone()}
	}
}

impl World {
	pub fn new() -> Self {
		World{ 
			l: Logger::new(Time::new()), 
			e_q: EventManager::new(),
		}
	}

	pub fn init(mut self, logger_:Logger) -> String{
		self.l = logger_;
		self.l.log("World initialization started...");

		let mut t:Terrain = Terrain::new(TerrainTypes::Field);
		self.l.log("Generating terrain...");
		t.make_terrain_base();
		let s:String = t.to_string();

		self.l.log("World initialization done.");
		s
	}
	pub fn update(_tick:i64){}
	pub fn save(){}
}