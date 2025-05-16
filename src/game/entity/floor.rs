use super::Entity;

pub struct Floor;

impl Floor {
	pub fn get_self(x:i64, y:i64, id:String) -> Entity{
		Entity {
			x,
			y,
			priority: super::Priority::LOW,
			self_: '.',
			id: id,
		}
	}
}