
use crate::game::entity::Entity;
pub(crate) struct MainLoop {
	tick:usize,
}

impl MainLoop {


	pub fn new() -> MainLoop {
		MainLoop { 
			tick: 0, 
		}
	}

	pub async fn main_loop(&mut self, mut entities:Vec<Entity>) -> (Vec<Entity>, usize) {
		let _tick_max = 20;//10f64.powf(127.0);
			
		//element que
		//gen world closure
		//gen one sub group
		//transform the sub groups


		loop {

			self.tick += 1;


			if self.tick < _tick_max {
				entities.push(Entity::new(0, self.tick, 'E')); // Replace with appropriate values for x, y, and self_
			} else {
				break;
			}
		}
		(entities, self.tick)
	}
}
