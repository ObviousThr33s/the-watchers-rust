use crate::game::entity::Entity;


pub struct Rectangle {
	width: usize,
	height: usize,
	entities: Vec<Entity>,
}

impl Rectangle {
	pub fn new(x:usize, y:usize, width: usize, height: usize) -> Self {
		let mut entities = Vec::new();
		for i in x..width {
			for j in y..height {
				entities.push(Entity::new(i, j, 'a'));
			}
		}
		Self { width, height, entities }
	}

	pub fn get_entities(&self) -> &Vec<Entity> {
		&self.entities
	}
}