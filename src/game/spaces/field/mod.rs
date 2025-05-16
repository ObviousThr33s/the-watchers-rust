use std::collections::HashMap;

use crate::game::entity::Entity;

/// Represents a 2D field where entities can be placed
pub struct Field {
	/// Map of entity IDs to their corresponding Entity objects
	pub entities: HashMap<String, Entity>,
	/// Map of entity positions and IDs to their corresponding Entity objects
	pub field: HashMap<(i64, i64, String), Entity>,
}

impl Clone for Field {
	fn clone(&self) -> Self {
		Self { 
			entities: self.entities.clone(),
			field: self.field.clone(),
		}
	}
}

impl ToString for Field {
	fn to_string(&self) -> String {
		let mut output: Vec<String> = Vec::new();
		output.push(String::from("\n"));

		for (id, _entity) in &self.entities {
			output.push(format!("\t{}\n", id));
		}

		output.concat()
	}
}

impl Field {
	/// Creates a new empty Field
	pub fn new() -> Self {
		Field { 
			entities: HashMap::new(), 
			field: HashMap::new(),
		}
	}

	/// Adds an entity to the field
	pub fn add_entity(&mut self, entity: Entity) {
		let key = entity.get();
		self.entities.insert(entity.id.clone(), entity.clone());
		self.field.insert(key, entity);
	}

	/// Updates an existing entity in the field with position changes
	pub fn set_entity(&mut self, entity: Entity) {
		// First, find and remove the old entity by ID
		if let Some(old_entity) = self.entities.get(&entity.id) {
			// Remove from the field map using old position
			self.field.remove(&old_entity.get());
		}

		// Update in the entities map
		self.entities.insert(entity.id.clone(), entity.clone());
		
		// Insert at the new position in the field map
		self.field.insert(entity.get(), entity);
	}
	
	/// Gets an entity at a specific position (x, y), if it exists
	pub fn get_entity_by_position(&self, x: i64, y: i64) -> Option<&Entity> {
		for ((entity_x, entity_y, _), entity) in &self.field {
			if *entity_x == x && *entity_y == y {
				return Some(entity);
			}
		}
		None
	}

	/// Gets an entity by its ID, if it exists
	pub fn get_entity_by_id(&self, id: &str) -> Option<&Entity> {
		self.entities.get(id)
	}

	pub fn get_entity_by_id_mut(&mut self, id:&str) -> Option<&mut Entity>{
		self.entities.get_mut(id)
	}

	pub fn remove_entity(&mut self, id: String) {
		if let Some(entity) = self.entities.remove(&id) {
			self.field.remove(&entity.get());
		}
	}
}
