
use crate::utils::logger::Logger;

use crate::game::world_generation::world_event::EventManager;
use crate::utils::time::Time;

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

	pub fn init(mut self, logger_:Logger, tick:i64){
		self.l = logger_;
		self.l.log("World initialization started...", tick);
		

		self.l.log("World initialization done.", tick);
	}
	pub fn update(_tick:i64){}
	pub fn save(){}
}