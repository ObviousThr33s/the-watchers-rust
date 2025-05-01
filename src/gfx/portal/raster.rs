
use std::collections::HashMap;

use super::pixel::{self, Pixel};

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
	pub fn new(self_:Vec<Pixel>) -> Self{
		Self{
			grid: HashMap::new()
		}
	}

	pub fn push(&mut self, pixel:Pixel) {
		self.grid.insert(pixel.p, pixel.c);
	}

	pub fn clear(&mut self){
		self.grid.clear();
	}

	pub fn to_string(&mut self, x:u16, y:u16) -> String {
		let mut s = String::new();

		for i in 0..y {
			for j in 0..x {
				s.push_str(self.grid.get(&(i, j)).map(String::as_str).unwrap_or(" "));
			}
			s.push_str("\n");
		}
		s
	}

}