use std::vec;

use rand::Rng;

use super::screen::Screen;

pub struct Lamp{
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

impl Clone for Lamp {
	fn clone(&self) -> Self { 
		let mut lamp_:Lamp = Lamp::init(self.lamp.x, self.lamp.y, CHARSETS::Charset0);
		lamp_.lamp = self.lamp.clone();
		lamp_
	} 
}

impl ToString for Lamp {
	fn to_string(&self) -> String {
		self.lamp.screen.iter().collect()
	}
}
impl Lamp {

	pub fn init(width:usize, height:usize, charset:CHARSETS) -> Self{
		let scr:Screen = Screen::new(width, height);
		let lamp_ = Lamp {			
			lamp: scr,
			charset: charset,
		};
	
		
		lamp_
	}
	
	pub fn get_charset(i:CHARSETS) -> String{
		let _charset_0:Vec<char> = vec!['*','.',',','+','=','-',' ', ' '];

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

	pub fn update(&mut self) {
	}

	pub fn make_lamp(&mut self) {

		self.lamp.screen.clear();

		let charset: String = Self::get_charset(self.charset);
		
		let s_x:usize = self.lamp.x;
		let s_y:usize = self.lamp.y;
		let _c_s:usize = charset.len();

		//turn into loc list at some point
		let _loc_x:i32 = 0;
		let _loc_y:i32 = 0;
	
		let mut _rng = rand::rng();
		
		let full_size = s_x*s_y;
		
		for i in 0..full_size-1 {
			let n_r = _rng.random_range(0..charset.len());
			let c: char = charset.chars().nth(n_r).unwrap_or(' ');

			self.lamp.screen.insert(i, c);
		}
		for j in 0..s_y+1{
			self.lamp.screen.insert(s_x*j, '\n');
		}
		

	}

	fn get_char(charset:CHARSETS, n:usize) -> char {
		if let Some(c) = Self::get_charset(charset).as_str().chars().nth(n) {
			c
		} else {
			' '
		}
	}

	
}