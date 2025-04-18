use std::fs::File;
use std::io::prelude::*;
use std::path::{self, Path, PathBuf};

pub struct FileData {
	pub file_path: PathBuf,
	pub id:usize
}

impl Clone for FileData {
	fn clone_from(&mut self, source: &Self) {
		*self = source.clone()
	}
	
	fn clone(&self) -> Self {
		Self { file_path: self.file_path.clone(), id: self.id.clone() }
	}
}

pub struct FileOutStream{
	files:Vec<FileData>
}
impl FileOutStream {
	pub fn new() -> FileOutStream{
		FileOutStream { files: Vec::new() }
	}
}

impl Clone for FileOutStream {
	fn clone(&self) -> Self {
		Self { files: self.files.clone() }
	}
}

pub trait FileOperations {
    fn write(&mut self, file:FileData, data: String);
	fn new(file_path:PathBuf, id_:usize) -> FileData;
	fn add_file(&mut self, file:FileData);

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
}

impl FileOperations for FileOutStream {

	fn new(filePath_:PathBuf, id_:usize) -> FileData {
		let f = FileData { 
							file_path: filePath_.clone(), 
							id: id_ 
						};
		f
	}


	fn write(&mut self, file: FileData, data: String) {
		let path: &Path = self.files[file.id].file_path.as_path();

		if !Self::is_valid_file_path(path.to_str().unwrap_or("invalid file path")) {
			let mut f = FileOutStream::create_file(String::from(path.to_str().unwrap()));
			if let Err(e) = f.write_all(data.as_bytes()) {
				eprintln!("Failed to write data to file: {}", e);
			}
		} else {
			if let Ok(mut f) = File::options().write(true).open(path) {
				if let Err(e) = f.write_all(data.as_bytes()) {
					eprintln!("Failed to write data to file: {}", e);
				}
			} else {
				eprintln!("Failed to open file for writing: {}", path.display());
			}
		}
	}
	
	fn add_file(&mut self, file:FileData) {
		self.files.push(file);
	}
}