use crate::game::event::entities::spaces::space::Space;

#[derive(Debug, Clone)]
pub struct World {
	pub space: Space,
}

impl World {
	pub fn new(space: Space) -> Self {
		Self { space }
	}
}