use std::vec;



use crate::game::{self};

use super::{charset::CHARSETS, screen::Screen};

pub struct Render{
	lamp: Screen,
	charset: CHARSETS,
}



impl Clone for Render {
	fn clone(&self) -> Self { 
		let mut lamp_:Render = Render::init(self.lamp.x, self.lamp.y, CHARSETS::Charset0);
		lamp_.lamp = self.lamp.clone();
		lamp_
	} 
}

impl ToString for Render {
	fn to_string(&self) -> String {
		self.lamp.screen.iter().collect()
	}
}

impl Render {

	pub fn init(width:usize, height:usize, charset:CHARSETS) -> Self{
		let scr:Screen = Screen::new(width, height);
		let lamp_ = Render {			
			lamp: scr,
			charset: charset,
		};
		
		
		
		lamp_
	}
	
	pub fn rasterize(&mut self, entities:Vec<game::entity::Entity>){ //add a screen buffer here{

		self.lamp.screen.clear();

		for i in 0..self.lamp.y {
			for j in 0..self.lamp.x {
				for e in entities.iter() {
					if e.x == j && e.y == i {
						self.lamp.screen.push(e.self_);
					}else {
						self.lamp.screen.push(' ');
					}
				}
			}
			self.lamp.screen.push('\n');
		}
	}	
}
