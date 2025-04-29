use super::Entity;

pub struct Floor {
}

impl Floor {
	pub fn get_self(x:usize, y:usize, id:String) -> Entity{
		Entity {
			x,
			y,
			priority: super::Priority::LOW,
			self_: '.',
			id: id,
		}
	}
}