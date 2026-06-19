use crate::game::entity::{Actions, Actor, Entity, EntityData};
use crate::game::entity::being::Being;

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
	pub fn new(x:i16, y:i16, name:String, id:String) -> Self{
		Self {
			entity: Entity::new(
				x,
				y,
				'F',
				id,
				crate::game::entity::Priority::MED,
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
	/// Overlay a `.being` definition onto this fairy: the file is the source of
	/// truth for name, stats, glyph, and art. This is the seam where the
	/// data-driven model starts driving the running game.
	pub fn apply_being(&mut self, being: &Being) {
		self.actor.name = being.name.clone();
		self.actor.health = being.health;
		self.actor.attack_power = being.power;
		self.actor.art = being.art.clone();
		self.actor.prompt = being.line.clone();
		self.entity.self_ = being.glyph;
	}

	pub fn warp(&mut self, tick:usize) {
		if tick%3 == 0 {
			self.entity.set_position(rand::random_range(0..10), rand::random_range(0..10));
			self.actor.health -= 1;
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
	fn attack(self, _actor: &mut Actor) {
		// TODO: combat is being reworked from HP toward light-as-resource;
		// left intentionally empty until the event system is settled.
	}
}