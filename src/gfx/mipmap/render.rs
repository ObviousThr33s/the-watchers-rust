use crate::{
	game::{entity::Entity, spaces::field::Field}, 
	gfx::{charset::CHARSETS, screen::Screen}
};

/// Handles rendering entities from a Field to a screen buffer
pub struct Render {
	render: Screen,
	charset: CHARSETS,
}

impl Clone for Render {
	fn clone(&self) -> Self { 
		Self {
			render: self.render.clone(),
			charset: self.charset,
		}
	} 
}

impl ToString for Render {
	fn to_string(&self) -> String {
		self.render.screen.iter().collect()
	}
}

impl Render {
	/// Creates a new render with specified dimensions and character set
	pub fn init(width: u16, height: u16, charset: CHARSETS) -> Self {
		Self {
			render: Screen::new(width, height),
			charset,
		}
	}
		/// Renders all entities from the field to the screen buffer
	pub fn rasterize(&mut self, field: &Field) {
		// Clear screen buffer
		self.render.screen.clear();
		
		// Pre-allocate with estimated capacity
		let estimated_capacity = (self.render.x as usize * self.render.y as usize / 10).max(field.entities.len());
		let mut position_map: std::collections::HashMap<(u16, u16), &Entity> = 
			std::collections::HashMap::with_capacity(estimated_capacity);

		// First pass: collect highest priority entity at each position
		for entity in field.entities.values() {
			let pos = (entity.x, entity.y);
			
			match position_map.get(&pos) {
				Some(existing) => {
					if entity.priority > existing.priority {
						position_map.insert(pos, entity);
					}
				},
				None => {
					position_map.insert(pos, entity);
				}
			}
		}
		
		// Pre-allocate screen buffer
		let total_size = (self.render.x as usize + 1) * self.render.y as usize; // +1 for newlines
		self.render.screen.reserve(total_size);
		
		// Render the screen
		for y in 0..self.render.y {
			for x in 0..self.render.x {
				// Check if an entity exists at this position
				if let Some(entity) = position_map.get(&(x, y)) {
					self.render.screen.push(entity.self_);
				} else {
					self.render.screen.push(' ');
				}
			}
			self.render.screen.push('\n');
		}
	}
}
