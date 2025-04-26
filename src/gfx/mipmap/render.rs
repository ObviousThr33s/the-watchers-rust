use crate::{game::{self}, gfx::{charset::CHARSETS, screen::Screen}};

pub struct Render{
	render: Screen,
	charset: CHARSETS,
}



impl Clone for Render {
	fn clone(&self) -> Self { 
		let render:Render = Render { render: self.render.clone(), charset: self.charset };
		render
	} 
}

impl ToString for Render {
	fn to_string(&self) -> String {
		self.render.screen.iter().collect()
	}
}

impl Render {

	pub fn init(width:usize, height:usize, charset:CHARSETS) -> Self{
		let scr:Screen = Screen::new(width, height);
		let lamp_ = Render {			
			render: scr,
			charset: charset,
		};
		
		
		
		lamp_
	}
	
	pub fn rasterize(&mut self, entities:Vec<game::entity::Entity>){ //add a screen buffer here{

		self.render.screen.clear();

		for i in 0..self.render.y {
			for j in 0..self.render.x {
				for e in entities.iter() {
					if e.x == j && e.y == i {
						self.render.screen.push(e.self_);
					}else {
						self.render.screen.push(' ');
					}
				}
			}
			self.render.screen.push('\n');
		}
	}	
}
