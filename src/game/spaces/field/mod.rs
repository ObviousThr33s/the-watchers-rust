
use std::collections::HashMap;

use crate::game::entity::{self, Entity};

pub struct Field{
	pub entities:HashMap<String,Entity>,
	pub field:HashMap<(usize, usize, String), Entity>
}

impl Clone for Field {
	fn clone(&self) -> Self {
		Self { 
			entities: self.entities.clone(),
			field:self.field.clone()
		}
	}
}

impl ToString for Field {
	fn to_string(&self) -> String {
		let mut s_:Vec<String> = Vec::new();
		s_.push(String::from("\n"));

		for i in self.entities.clone() {
			s_.push(format!("\t{}\n", i.0));
		}

		let s = s_.concat();
		s
	}
}

impl Field{

	pub fn new() -> Self{
		Field { entities: HashMap::new(), field: HashMap::new()}
	}

	pub fn add_entity(&mut self, entity:Entity){
		self.field.insert(entity.get(), entity);
	}

	pub fn set_entity(&mut self, entity:Entity) {
		self.field.get(&entity.get()).replace(&entity);
	}
}
