
use std::char;

use crate::gfx::charset::{self, CHARSETS};

use super::{pixel::Pixel, raster::Raster};

pub struct View {
	view_vector:Raster,
	pub port:Vec<char>,
	pub char_set:CHARSETS
}

impl Clone for View {
	fn clone(&self) -> Self {
		Self { 
			view_vector: self.view_vector.clone(),
			port:self.port.clone(),
			char_set:self.char_set.clone()
		}
	}
}

impl View {

	pub fn new() -> Self {
		Self { 
			view_vector:Raster::new(),
			port: Vec::new(),
			char_set:CHARSETS::Charset0
		}
	}

	pub fn init(&mut self) {
		self.fill_raster_glyphs();
	}

	pub fn get_as_glyphs(mut self) -> Vec<char>{
		for i in self.view_vector.raster {
			self.port.push(super::Portal::get_char_from_angle(i.angle));
		}

		self.port.clone()
	}

	fn fill_shade_at(x:u16,y:u16,angle:u8){

	}

	fn fill_raster_glyphs(&mut self){
		let mut a:u8 = 0;
		let char_set_len:usize = charset::get_charset_vec(self.char_set).len();
		
		for i in 0..480{
			for j in 0..640{
				self.view_vector.raster.push(Pixel{
					x:i,
					y:j,
					angle:a,
				});
				if a > (char_set_len-3) as u8 {
					a = 0u8;
				}else{
					a += 1u8;
				}
			}
		}
	}
}