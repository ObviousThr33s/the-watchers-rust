pub struct Pixel{
	pub wall_height:f64,
	pub distance:f64,
	pub id:String
}


impl Clone for Pixel {
	fn clone(&self) -> Self {
		Self { wall_height: self.wall_height.clone(), distance: self.distance.clone(), id: self.id.clone() }
	}
}

impl Pixel{
	pub fn new(wall_height:f64, distnace:f64, id:String) -> Self {
		Self{
			wall_height:wall_height,
			distance:distnace,
			id:id
		}
	}
}

