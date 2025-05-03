
use std::collections::HashMap;

use super::pixel::Pixel;

#[warn(dead_code)]
pub struct Raster {
	grid:HashMap<(u16,u16), String>
}

impl Clone for Raster {
	fn clone(&self) -> Self {
		Self { grid: self.grid.clone() }
	}
}

impl Raster {
	pub fn new(_self:Vec<Pixel>) -> Self{
		Self{
			grid: HashMap::new()
		}
	}

}