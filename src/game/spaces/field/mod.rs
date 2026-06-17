use std::collections::HashMap;

use crate::game::entity::Entity;

/// Represents a 2D field where entities can be placed
pub struct Field {
	/// Map of entity IDs to their corresponding Entity objects
	pub entities: HashMap<String, Entity>,
	/// Spatial index for fast position-based lookups: (x, y) -> entity_id
	pub spatial_index: HashMap<(i16, i16), String>,
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
	pub fn get_entity_by_position(&self, x: i16, y: i16) -> Option<&Entity> {
		self.spatial_index.get(&(x, y))
			.and_then(|id| self.entities.get(id))
	}

	/// Returns true if some entity other than `ignore_id` occupies (x, y).
	/// Every entity is treated as solid, matching how the ray caster renders
	/// them all as walls.
	#[inline]
	pub fn is_occupied(&self, x: i16, y: i16, ignore_id: &str) -> bool {
		self.get_entity_by_position(x, y)
			.is_some_and(|e| e.id != ignore_id)
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

	/// Renders entities to an ASCII map of the given size, scrolled so that
	/// (center_x, center_y) sits in the middle. Centering on the player keeps
	/// them pinned to the middle of the minimap as they move around the world.
	pub fn to_ascii_map(&self, width: usize, height: usize, center_x: i16, center_y: i16) -> String {
		let mut output = vec![vec![' '; width]; height];

		// World coordinate of the visible window's top-left corner.
		let origin_x = center_x - (width as i16) / 2;
		let origin_y = center_y - (height as i16) / 2;

		for entity in self.entities.values() {
			let sx = entity.x - origin_x;
			let sy = entity.y - origin_y;

			// Only draw entities that fall inside the visible window.
			if sx >= 0 && (sx as usize) < width && sy >= 0 && (sy as usize) < height {
				output[sy as usize][sx as usize] = entity.self_;
			}
		}

		output
			.iter()
			.map(|row| row.iter().collect::<String>())
			.collect::<Vec<_>>()
			.join("\n")
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::game::entity::{Entity, Priority};

	#[test]
	fn minimap_centers_on_the_given_point() {
		let mut field = Field::new();
		field.add_entity(Entity::new(5, 5, '^', "Player".to_string(), Priority::MED));
		field.add_entity(Entity::new(5, 4, '#', "wall".to_string(), Priority::LOW));

		// A 5x5 window centered on (5,5) has origin (3,3), so the player lands
		// dead center at (2,2) and the wall one row above it at (2,1).
		let map = field.to_ascii_map(5, 5, 5, 5);
		let rows: Vec<&str> = map.lines().collect();

		assert_eq!(rows[2].chars().nth(2), Some('^'), "player should be centered");
		assert_eq!(rows[1].chars().nth(2), Some('#'), "wall should sit one row above");
	}
}
