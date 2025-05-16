use crate::game::entity::{Actions, Actor, Entity, EntityData};

pub struct Fairy {
	pub entity:Entity,
	pub actor:Actor
}

impl Clone for Fairy {
	fn clone(&self) -> Self {
		Self {
			entity: self.entity.clone(),
			actor: self.actor.clone()
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
			),
			actor:Actor {
				name: name,
				health: 10,
				attack_power:10,
				art:String::new(),
				prompt:String::new(),
			} 
		}
	}
}

impl Fairy {
	pub fn warp(&mut self, tick:usize) {
		if tick%7 == 0 {
			self.entity.set_position(rand::random_range(0..10), rand::random_range(0..10));
		}
	}
}

impl EntityData for Fairy {
	fn get_health(self) -> i32{
		self.actor.health
	}

	fn set_health(&mut self, health:i32) {
		self.actor.health = health;
	}
	
	fn get_power(self) -> i32 {
		self.actor.attack_power
	}
	
	fn set_power(&mut self, attack_power:i32) {
		self.actor.attack_power = attack_power
	}
}

impl Actions for Fairy {
	fn attack(self, actor: &mut Actor) {
		actor.health = actor.health-self.get_power();
	}
}