use super::pixel::Pixel;

#[warn(dead_code)]
pub struct Raster {
	pub chars:Vec<char>,
	queue:Vec<Pixel>
}

impl Raster {
	pub fn new() -> Self{
		Self{
			chars: Vec::new(),
			queue: Vec::new()
		}
	}
	
	pub fn place(){

	}

	pub fn update(){

	}
}