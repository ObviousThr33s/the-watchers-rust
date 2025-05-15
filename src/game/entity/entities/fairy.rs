use crate::game::entity::{Entity, Actor, Actions};
use rand::rngs::ThreadRng;

pub struct Fairy {
	pub entity:Entity,
}

impl Clone for Fairy {
	fn clone(&self) -> Self {
		Self {
			entity: self.entity.clone(),
		}
	}
}

impl Fairy {
	pub fn new(x:i64, y:i64, name:String, id:String) -> Self{
		Self {
			entity: Entity::new(
				x,
				y,
				'F',
				id,
				crate::game::entity::Priority::MED, // Assuming there's a Priority enum, adjust as needed
				Actor {
					name: name,
					health: 10,
					attack_power:10,
					art:String::new()
				} 
			)
		}
	}
}

impl Actions for Fairy {
	fn warp(&mut self) {
		self.entity.set_position(rand::random_range(0..4), rand::random_range(0..4));
	}
}