
use raster::Raster;

use crate::game::{entity::{player::Player, wall_type::WallType}, spaces::field::Field};

use super::screen::Screen;

pub mod raster;
pub mod pixel;

pub struct Portal {
	pub screen:Screen,
	pub raster:Raster,
}

impl Portal {

	pub fn new() -> Self {
		Self { screen: Screen::new(0, 0), raster: Raster::new()}
	}
	pub fn update_raster_walls(&mut self, field:Field, player:&Player) {
		// Clear existing walls
		self.raster.clear();
		
		// Create boundary walls of stone (keeping this for world boundaries)

		// Add walls based on entities in the field
		for (_, entity) in field.entities.iter() {
			// Skip the player entity (don't want player to be a wall)
			if entity.id == player.player.id {
				continue;
			}
			
			// Determine wall type based on entity properties
			let wall_type = WallType::determine_wall_type(&entity);
			
			// Add to raster
			self.raster.add_wall_point(entity.x as u16, entity.y as u16, wall_type);
		}
	}

}