pub struct Pixel{
	pub p:(u16,u16),
	pub c:String
}


impl Clone for Pixel {
	fn clone(&self) -> Self {
		Self { p: self.p.clone(), c: self.c.clone() }
	}
}

impl Pixel{
	pub fn new(x:u16,y:u16,c:String) -> Self {
		Self { p: (x,y), c:c }
	}

	pub fn get_self(&self) -> (u16, u16, String) {
		(self.p.0, self.p.1, self.c.clone())
	}
}

