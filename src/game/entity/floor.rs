use super::{actor::Actor, Entity};

pub struct Floor;

impl Floor {
	pub fn get_self(x:i64, y:i64, id:String) -> Entity{
		Entity {
			x,
			y,
			priority: super::Priority::LOW,
			self_: '.',
			id: id,
			actor: Actor { name: "Floor".to_owned(), health: 1, attack_power:0, art:String::new() }
		}
	}
}