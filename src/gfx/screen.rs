//just playing around with structs here
pub struct Screen{
	pub x:u16,
	pub y:u16,
	pub screen: Vec<char>,
}

//This structure holds the raw data for each rendered widget which is given by
//each renderer (exept for the log data)

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

	pub fn new(width:u16, height:u16) -> Self{
		let screen = Screen{
			x:width,
			y:height,
			screen: Vec::new(),
		};

		screen
	}
	
	pub fn to_string_break(&self) -> String {
		let mut s:String = String::new();
		let mut j = 0;

		for c in self.screen.clone() {
			s.push(c);
			j += 1;
			if j == self.x {
				s.push('\n');
				j = 0;
			}
		}
		s
	}

	pub fn from_string(&mut self, s:String){
		self.screen.clear();
		self.screen = s.chars().collect();
	}

	pub fn _get_size(&self) -> u16{
		self.x*self.y
	}
}
