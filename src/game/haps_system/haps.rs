pub struct Haps {
	ev: Vec<()>
}

impl Haps {
	pub fn new() -> Self {
		Self{
			ev: Vec::new()
		}
	}
	
	pub fn add_event(&mut self, event:()) {
		self.ev.push(event);
	}

	pub fn execute(&self) -> () {
		for e in &self.ev {
			*e
		}
	}
}