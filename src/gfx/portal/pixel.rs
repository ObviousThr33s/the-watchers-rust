pub struct Pixel {
	pub value: u8,
	pub color: u128,
}

impl Clone for Pixel {
	fn clone(&self) -> Self {
		Self {
			value: self.value,
			color: self.color,
		}
	}
}

impl Pixel {
	pub fn new(value: u8, color: u128) -> Self {
		Self { value, color }
	}
}