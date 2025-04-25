use std::vec;



use crate::game::{self, entity::{self, Entity}};

use super::screen::Screen;

pub struct Render{
	pub lamp: Screen,
	pub charset: CHARSETS,
}

#[derive(Copy, Clone)]
pub enum CHARSETS {
	Charset0 = 0,
	Charset1 = 1,
	Charset2 = 2,
	Charset3 = 3
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
	
	pub fn get_charset(i:CHARSETS) -> String{
		let _charset_0:Vec<char> = vec!['.',',','-','=','+','*','#','▓'];

		let _charset_1:Vec<char> = vec!['░','▒','▓'];

		let _charset_2:Vec<char> = vec!['a','b','c','d','e','f','g','h','i','j',
									'k','l','m','n','o','p','q','r','s','t',
									'u','v','w','x','y','z'];
		
		let _charset_3:Vec<char> = vec!['a','b','c','d','e','f','g','h','i','j',
								 'k','l','m','n','o','p','q','r','s','t',
								 'u','v','w','x','y','z','1','2','3','4',
								 '5','6','7','8','9','0','#','*','.',',',
								 '+','=','-',' ',' ',' ',' ',' ',' ',' ',

								 ' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',
								 ' ',' ',' ',' '];
		
		
		let sets:Vec<_> = vec![_charset_0, _charset_1, _charset_2, _charset_3];
		
		let r = match i {
			CHARSETS::Charset0 => sets.get(0).unwrap(),
			CHARSETS::Charset1 => sets.get(1).unwrap(),
			CHARSETS::Charset2 => sets.get(2).unwrap(),
			CHARSETS::Charset3 => sets.get(3).unwrap(),
		};
		
		r.iter().collect::<String>()

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
