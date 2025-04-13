use super::file_object::{self, FileObject};

pub struct  SaveFile {
	player_id:i32,
	hp:i64,
	
	x:i128,
	y:i128,
	z:i128,

	game_file: file_object::GameFile,

}

pub  trait  SaveFileObject {
	fn create(self) -> Self;
	fn new() -> Self;
	fn save(self);
}


impl SaveFileObject for SaveFile{
	fn create(self) -> Self {
		 SaveFile { 
			player_id: self.player_id, 
			hp: self.hp, 
			
			x: self.x, 
			y: self.y, 
			z: self.z,
			game_file: self.game_file.clone()
		}
	}
	fn new() -> Self {
		SaveFile { 
			player_id: 0, 
			hp: 0, 
			x: 0, 
			y: 0, 
			z: 0, 
			game_file:FileObject::new() 
		}
	}
	fn save(self) {
		
	}
	
}