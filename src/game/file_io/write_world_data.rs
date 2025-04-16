use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

struct FileData {
	filepath:Path::Path,
}

impl WriteWorldData{
	pub fn write(self, path_to:String, data:String){
		self.filepath = Path::new(path_to);
		
		let mut file = match File::open(&path) {
			Err(why) => create_file(path_to),
			Ok(file) => file,
		};
	}

	pub fn create_file(path_to:String){
		let mut file = match File::create(&path_to) {
			Err(why) => panic!("couldn't create {}: {}", display, why),
			Ok(file) => file,
		};
	}
}