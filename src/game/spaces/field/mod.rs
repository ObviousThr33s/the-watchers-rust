use std::collections::HashMap;

use crate::game::entity::Entity;

/// Represents a 2D field where entities can be placed
pub struct Field {
	/// Map of entity IDs to their corresponding Entity objects
	pub entities: HashMap<String, Entity>,
	/// Spatial index for fast position-based lookups: (x, y) -> entity_id
	pub spatial_index: HashMap<(u16, u16), String>,
}

impl Clone for Field {
	fn clone(&self) -> Self {
		Self { 
			entities: self.entities.clone(),
			spatial_index: self.spatial_index.clone(),
		}
	}
}

impl ToString for Field {
	fn to_string(&self) -> String {
		let mut output: Vec<String> = Vec::with_capacity(self.entities.len() + 1);
		output.push(String::from("\n"));

		for id in self.entities.keys() {
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
			spatial_index: HashMap::new(),
		}
	}

	/// Adds an entity to the field
	pub fn add_entity(&mut self, entity: Entity) {
		let pos = (entity.x, entity.y);
		let id = entity.id.clone();
		
		// Remove any existing entity at this position
		if let Some(old_id) = self.spatial_index.insert(pos, id.clone()) {
			self.entities.remove(&old_id);
		}
		
		self.entities.insert(id, entity);
	}

	/// Updates an existing entity in the field with position changes
	pub fn set_entity(&mut self, entity: Entity) {
		let new_pos = (entity.x, entity.y);
		let id = &entity.id;
		
		// Remove old spatial index entry if entity exists and position changed
		if let Some(old_entity) = self.entities.get(id) {
			let old_pos = (old_entity.x, old_entity.y);
			if old_pos != new_pos {
				self.spatial_index.remove(&old_pos);
			}
		}

		// Update spatial index and entity
		self.spatial_index.insert(new_pos, id.clone());
		self.entities.insert(id.clone(), entity);
	}
	
	/// Gets an entity at a specific position (x, y), if it exists
	#[inline]
	pub fn get_entity_by_position(&self, x: u16, y: u16) -> Option<&Entity> {
		self.spatial_index.get(&(x, y))
			.and_then(|id| self.entities.get(id))
	}

	/// Gets an entity by its ID, if it exists
	#[inline]
	pub fn get_entity_by_id(&self, id: &str) -> Option<&Entity> {
		self.entities.get(id)
	}

	#[inline]
	pub fn get_entity_by_id_mut(&mut self, id: &str) -> Option<&mut Entity> {
		self.entities.get_mut(id)
	}

	pub fn remove_entity(&mut self, id: String) {
		if let Some(entity) = self.entities.remove(&id) {
			self.spatial_index.remove(&(entity.x, entity.y));
		}
	}
}
