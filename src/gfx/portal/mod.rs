

use view::View;

use super::{charset::CHARSETS, screen::Screen};

pub mod pixel;
pub mod raster;
pub mod view;

pub struct Portal{
	char_set:CHARSETS,
	pub screen:Screen
}

impl Clone for Portal {
	fn clone(&self) -> Self {
		Self { char_set: self.char_set.clone(), screen: self.screen.clone() }
	}
}

impl Portal {

	pub fn new() -> Self {
		Self { char_set: CHARSETS::Charset0, screen: Screen::new(640, 480) }
	}

	pub fn init(&mut self) {
		let mut v:View = View::new();
		v.init();

		self.screen.screen.clear();
		for i in v.get_as_glyphs(){
			self.screen.screen.push(i);
		}
	}

	pub fn get_char_from_angle(angle:u8) -> char {
		let char_set:Vec<char> = vec!['.',',','-','=','░','▒','▓'];
		
		let c = char_set[angle as usize];

		c
	}
}