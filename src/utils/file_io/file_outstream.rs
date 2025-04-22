use std::{fs::File, io::Write, path::{Path, PathBuf}};

use super::{file_data::{self, FileData}, FileOperations};

pub struct FileOutStream{
	files:Vec<FileData>,
	tick:usize
}
impl FileOutStream {
	pub fn new() -> FileOutStream{
		FileOutStream { files: Vec::new(), tick:0 }
	}
}

impl Clone for FileOutStream {
	fn clone(&self) -> Self {
		Self { files: self.files.clone(), tick: self.tick.clone() }
	}
}

impl FileOperations for FileOutStream {

	fn new(_file_path:String, file_name:String, id_:usize, data:String) -> FileData {
		let file_outstream = FileOutStream::new();
		let file_path: PathBuf = file_outstream.create_file_path(&file_name);
		
		let f = FileData { 
			file_path: file_path.to_string_lossy().to_string(), 
			id: id_,
			file_name: file_name.clone(),
    		file_data: data
		};
		
		// Removed unused variable `fp_as_string`
		
		// Ensure the directory exists
		if let Some(parent_dir) = file_path.parent() {
			std::fs::create_dir_all(parent_dir).expect("Failed to create log directory");
		}
		
		f
	}

	
	fn write_all(&mut self) {
		for f in 0..self.files.len() {
			let file_data = self.files[f].clone(); // Clone the FileData to avoid borrowing issues
			self.write(file_data);
		}
	}

	fn write(&mut self, file: FileData) {
		let file_path = format!("{}\\{}.txt", file.file_path, file.file_name);
		let path: &Path = Path::new(&file_path);

		if Self::is_valid_file_path(file.file_path.as_str()) {
			let mut f = File::create(path).expect("Failed to create or open file");
			f.write_all(file.file_data.as_bytes()).expect("Failed to write to file");
			f.flush().expect("Failed to flush file");
		}
	}
	
	fn add_file(&mut self, _file_path:String, file_name:String) {
		let f:FileData = FileData { 
			file_path: _file_path, 
			file_name: format!("{}_{}.txt",file_name, self.tick),  
			file_data: self.files.iter().map(|file| file.file_data.clone()).collect::<String>(),

			id:self.tick,

		};

		self.tick += 1;

		self.files.push(f);
	}

}