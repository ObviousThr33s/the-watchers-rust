use std::fs::File;
use std::path::{Path, PathBuf};
use file_data::FileData;

pub mod file_data;
pub mod file_outstream;

pub trait FileOperations {
    fn write(&mut self, file:FileData);
	fn new(file_path:String, file_name:String, id_:usize, date:String) -> FileData;
	fn add_file(&mut self, file_path:String, file_name:String);
	fn write_all(&mut self);

	fn create_file(path_to:String) -> File{
		let file = match File::create(&path_to) {
			Err(why) => panic!("couldn't create {}: {}", path_to, why),
			Ok(file) => file,
		};
		file
	}

	fn is_valid_file_path(file_path: &str) -> bool {
		let path = Path::new(file_path);
		path.exists() && path.is_file()
	}

	fn create_file_path(&self, file_name: &str) -> PathBuf {
		let mut path = std::env::current_dir().expect("Failed to get current directory");
		path.push(file_name);
		path
	}
	
}

