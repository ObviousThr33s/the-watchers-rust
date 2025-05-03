pub struct Pixel {
	pub x: u16,
	pub y: u16,
	pub color: u128,
	pub char: char,
}

impl Clone for Pixel {
	fn clone(&self) -> Self {
		Self {
			x: self.x,
			y: self.y,
			color: self.color,
			char: self.char,
		}
	}
}

impl Pixel {
	pub fn new(x: u16, y: u16, color: u128, char: char) -> Self {
		Self { x, y, color, char }
	}
}