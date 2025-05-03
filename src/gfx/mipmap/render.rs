use crate::{
	game::{entity::{Entity, Priority}, spaces::field::Field}, 
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
	pub fn init(width: usize, height: usize, charset: CHARSETS) -> Self {
		Self {
			render: Screen::new(width, height),
			charset,
		}
	}
	
	/// Renders all entities from the field to the screen buffer
	pub fn rasterize(&mut self, field: &Field) {
		// Clear screen buffer
		self.render.screen.clear();
		
		// Create a raster with only the highest priority entity at each position
		let mut position_map: std::collections::HashMap<(usize, usize), &Entity> = std::collections::HashMap::new();
		
		// First pass: collect highest priority entity at each position
		for (_, entity) in field.entities.iter() {
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
