
use crate::game::event::event_manager::EventManager;
use crate::game::transforms::terrain;
use crate::utils::logger::Logger;
use crate::game::transforms::generators::gen_field::gen_field;


use super::space::Space;

pub struct World {
	event_q:EventManager,
	Terrain:terrain::Terrain,
}

impl Clone for World{
	fn clone(&self) -> Self {
		Self { 
			event_q: self.event_q.clone(), 
			Terrain: self.Terrain.clone(),
		}
	}
}

impl Space for World {
	fn new() -> Self {
		World{
			event_q: EventManager::new(),
			Terrain: terrain::Terrain::new(terrain::TerrainTypes::Field),
		}
	}

	fn init(self, logger_:&mut Logger) -> World{
		logger_.log("World initialization started...");

		let terrain_depth:i8 = 3;

		let mut terrain = terrain::Terrain::new(terrain::TerrainTypes::Field);
		gen_field(&mut terrain, terrain_depth);

		logger_.log("Terrain generated...");

		logger_.log("World initialization done.");
		
		self
	}
	fn update(self, _tick:i64) -> Self{
		self
	}
	fn save(self){
	}
}