
use raster::Raster;

use crate::game::entity::player::Player;

use super::screen::Screen;

pub mod raster;
pub mod pixel;

pub struct Portal {
	pub screen:Screen,
	pub raster:Raster,
	pub rays:Vec<(f64,f64)>
}

impl Portal {

	pub fn new() -> Self {
		Self { screen: Screen::new(0, 0), raster: Raster::new(Vec::new()), rays:Vec::new() }
	}

	pub fn create_raster(&mut self, _player:Player, w:usize, h:usize) {

	}

}