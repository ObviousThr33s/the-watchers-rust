use noise::core::value;
use pixel::Pixel;

use crate::utils::logger;

use super::screen::Screen;

pub mod pixel;

pub struct Portal {
	pub screen:Screen,
	pub art:String,
	pub prompt:String,
}

impl Portal {

	pub fn new() -> Self {
		Self { screen: Screen::new(0, 0), art:"none".to_owned(), prompt:"none".to_owned()}
	}
	
	pub fn set_portal(&mut self, art:String, prompt:String) {
		self.art = art;
		self.prompt = prompt;
	}

	pub fn build_screen(&mut self, height:u16, width:u16) {
		let screen_lines = self.create_raster_vector(width,height);
		self.screen.screen.clear();
		
		for i in 0..height {
			for j in 0..width {
				self.screen.screen.push(screen_lines[i as usize].chars().nth(j as usize).unwrap());
			}
			self.screen.screen.push('\n');
		}
		
	}

	fn create_raster_vector(&mut self, width:u16, height:u16) -> Vec<String> {
		self.screen.x = width;
		self.screen.y = height;
		self.screen.screen.clear();
		
		let mut pixels:Vec<Pixel> = Vec::new();
		
		for y in 0..height {
			for x in 0..width {
				let pixel = Pixel::new(1, 0);
				pixels.push(pixel);
			}
		}

		self.build_vector_screen(pixels.clone(), width, height)
	}	

	fn build_vector_screen(&mut self, pixels: Vec<Pixel>, width:u16, height:u16) -> Vec<String>{
		let mut screen:Vec<String> = Vec::new();
		let mut line:String = String::new();
		let mut k:usize = 0;

		for i in 0..height {
			for j in 0..width {
				line.push(Self::get_char_from_value(pixels[k].value).clone());
				k += 1
			}
			screen.push(line);

			line = String::new();
		}

		screen

	}

	fn get_char_from_value(value: u8) -> char {
		match value {
			0 => '█',
			1 => '▒',
			2 => '░',
			3 => ' ',
			_ => ' ',
		}
	}
}