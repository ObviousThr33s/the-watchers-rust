//just playing around with structs here
pub struct Screen{
	pub x:usize,
	pub y:usize,
	pub screen: Vec<char>,
}

impl Clone for Screen {
	fn clone(&self) -> Screen{

		let scr:Vec<char> = self.screen.clone();

		let scr= Screen {
			x : self.x,
			y : self.y,
			screen : scr
		};
		scr
	}
}

impl ToString for Screen{
	fn to_string(&self) -> String {
		let mut s:String = String::new();

		for i in self.screen.iter().into_iter(){
			s.insert(0,i.clone());
		}

		s
	}
}

impl Screen{
	
	pub fn new(width:usize, height:usize) -> Self{
		let screen = Screen{
			x:width,
			y:height,
			screen: Vec::new(),
		};

		screen
	}
	
	pub fn _get_size(&self) -> usize{
		self.x*self.y
	}
}
