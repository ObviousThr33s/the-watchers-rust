use crate::game::entity::Entity;
pub struct Rectangle {
	width: usize,
	height: usize,
	entities: Vec<Entity>,
}

impl Rectangle {

	pub fn get_entities(self) -> Vec<Entity> {
		self.entities.clone()
	}

	pub fn get_width(self) -> usize{
		self.width.clone()
	}

	pub fn get_height(self) -> usize{
		self.height.clone()
	}


}