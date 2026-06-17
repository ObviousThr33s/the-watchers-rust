/// Raw character buffer for a rendered widget. The minimap renderer fills
/// `screen` cell-by-cell and reads it back out for display.
pub struct Screen {
	pub x: u16,
	pub y: u16,
	pub screen: Vec<char>,
}

impl Clone for Screen {
	fn clone(&self) -> Screen {
		Screen {
			x: self.x,
			y: self.y,
			screen: self.screen.clone(),
		}
	}
}

impl Screen {
	pub fn new(width: u16, height: u16) -> Self {
		Screen {
			x: width,
			y: height,
			screen: Vec::new(),
		}
	}
}
