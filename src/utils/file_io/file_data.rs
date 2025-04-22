use std::clone;

pub struct FileData {
	pub file_path: String,
	pub file_name: String,
	pub id:usize,

	pub file_data: String,
}

impl Clone for FileData {
	fn clone(&self) -> Self {
		Self {  file_path: self.file_path.clone(), 
				file_name: self.file_name.clone(), 
				id: self.id.clone(), 

				file_data: self.file_data.clone()
			}
	}
}

impl FileData {
	
}