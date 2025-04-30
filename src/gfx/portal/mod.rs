
use std::{char, ops::Index};
use noise::NoiseFn;

use chrono::offset;
use raster::Raster;

use super::{charset, screen::{self, Screen}};

pub mod raster;
pub mod pixel;

pub struct Portal {
	pub screen:Screen,
	pub raster:Raster
}

impl Portal {

	pub fn new() -> Self {
		Self { screen: Screen::new(0, 0), raster: Raster::new() }
	}

	pub fn fill_raster(&mut self, width:u16, height:u16, tick:usize){
		self.raster.chars.clear();

		let a:Vec<char> = charset::get_charset_vec(charset::CHARSETS::Charset0);
		
		self.raster.chars.clear();

		for i in 0..height*width {
			self.raster.chars.push(a[tick]);
		}
	}

	pub fn make_screen(&mut self, width:u16, height:u16){
		self.screen.screen.clear();
	
		let mut k = 0;

		for i in 0..height {
			for j in 0..width {
				self.screen.screen.push(self.raster.chars[k]);
				k += 1;
			}
			if k%8 == 0 {
				self.screen.screen.push('\n');
			}
		}
	}
}