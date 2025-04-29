use super::pixel::Pixel;

pub struct Raster {
	pub raster:Vec<Pixel>
}

impl Clone for Raster {
	fn clone(&self) -> Self {
		Self { raster: self.raster.clone() }
	}
}

impl Raster {
	pub fn new() -> Self {
		Self{
			raster:Vec::new()
		}
	}
}