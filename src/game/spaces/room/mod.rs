use crate::game::entity::{floor::Floor, Entity};
pub struct Room {
	width: usize,
	height: usize,
	_entities: Vec<Entity>,
}

impl Room {
	pub fn gen_floor(height:usize, width:usize, x:usize, y:usize, id:String) -> Vec<Entity> {
		let mut f:Vec<Entity> = Vec::new();

		for i in y..height+y {
			for j in x..width+x {
				f.push(Floor::get_self(i,j, format!("{}{}{}",id.clone(),i,j)));
			}
		}
		f
	}
	

	pub fn get_width(self) -> usize{
		self.width.clone()
	}

	pub fn get_height(self) -> usize{
		self.height.clone()
	}


}