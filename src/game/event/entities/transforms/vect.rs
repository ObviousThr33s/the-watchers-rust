use std::clone;

pub struct Vect {
	grade:Vec<u8>,
	char_map:Vec<char>,
	vect:Vec<u8>
}

impl Clone for Vect{
	fn clone(&self) -> Self {
		Self { grade: self.grade.clone(), char_map: self.char_map.clone(), vect: self.vect.clone() }
	}
}

impl Vect {
	pub fn new() -> Self{
		Self { 
			grade: Vec::new(),
			char_map: Vec::new(),
			vect: Vec::new()
		}
	}

	pub fn populate(&mut self, vect:Vec<u8>) -> Self{
		//implicit location in pairs
		for i in vect {
			self.vect.push(i);
		}

		self.clone()
	}

}