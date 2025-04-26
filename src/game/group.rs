use std::collections::HashMap;

use super::entity::{self, Entity};

pub struct Group {
	pub entities:HashMap<String,Entity>
}

impl Clone for Group {
	fn clone(&self) -> Self {
		Self { entities: self.entities.clone() }
	}
}

impl Group {
	pub fn new() -> Self {
		Self { entities:HashMap::new() }
	}
	
	
	pub fn get_entity(&mut self, s: String) -> Option<&mut Entity> {
		self.entities.get_mut(&s)
	}
}