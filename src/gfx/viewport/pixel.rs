pub struct Pixel {
	x:u64,
	y:u64,
	angle:u8,
	self_:char
}

impl Pixe{

	pub fn new(x:u16, y:u16, angle:u8){
		Self {
			x:x,
			y:y,
			angle:angle,
			self_:'X'
		}
	}

	pub fn set_angle(angle:u8){
		self.angle = angle
	}

	pub fn set_self(c:char){
		self_ = c;
	}
}

