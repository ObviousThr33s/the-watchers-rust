use super::Entity;

pub struct Floor;

impl Floor {
	pub fn get_self(x:i16, y:i16, id:String) -> Entity{
		Entity {
			x,
			y,
			priority: super::Priority::LOW,
			self_: '.',
			id: id,
		}
	}
}