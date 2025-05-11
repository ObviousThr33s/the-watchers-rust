
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;


use super::time::Time;


pub struct Logger {
	pub tick:usize,
	pub start_time: Time,

	log_stream: Vec<String>,
	vers:String
}

impl Clone for Logger {
	fn clone(&self) -> Self {
		Self {
				log_stream: self.log_stream.clone(), 
				start_time: self.start_time.clone(),
				tick: self.tick.clone(),
				vers: self.vers.clone()
		}
	}
}

impl Logger {

	pub fn new(start_time:Time, version:String) -> Self {
		Logger { 
			log_stream: Vec::new(),
			start_time: start_time,
			tick: 0,
			vers: version
		}
	}

	pub fn log(&mut self, message: &str) {
		let s0: String = self.start_time.to_string();
		self.log_stream.push(format!("[{}]<{}> {}\n", s0, self.tick, message));
		
		self.tick += 1;
	}

	pub fn get_log(self) -> Vec<String> {
		let mut log_:Vec<String> = Vec::new();
		
		for i in self.log_stream{
			log_.push(i.clone());
		}
		log_.reverse();

		log_
	}

	pub fn get_version(self) -> String {
		self.vers
	}


	//saves a special log to a specific directory
	pub fn save_log_sp(dir:&str, file_name:&str, message:String){
		let dir = format!("./{}/", dir);
		let file_name = format!("{}.txt", file_name);

		let mut file_path = PathBuf::from(dir.clone());
		file_path.push(file_name);

		
		if let Err(e) = std::fs::create_dir_all(dir) {
			eprintln!("Failed to create directory: {}", e);
			return;
		}

		let mut f: File = match File::create(file_path) {
			Ok(file) => file,
			Err(e) => {
				eprintln!("Failed to create file: {}", e);
				return;
			}
		};

		let log_data = message;
		if let Err(e) = f.write_all(&log_data.as_bytes()) {
			eprintln!("Failed to write to file: {}", e);
		}
	}

	//saves the main log
	pub fn save_log(&mut self) {
		let dir = "./res/logs";
		let file_name = "log.txt";

		let mut file_path = PathBuf::from(dir);
		file_path.push(file_name);

		
		if let Err(e) = std::fs::create_dir_all(dir) {
			eprintln!("Failed to create directory: {}", e);
			return;
		}

		let mut f: File = match File::create(file_path) {
			Ok(file) => file,
			Err(e) => {
				eprintln!("Failed to create file: {}", e);
				return;
			}
		};

		let log_data = self.log_stream.join("").into_bytes();
		if let Err(e) = f.write_all(&log_data) {
			eprintln!("Failed to write to file: {}", e);
		}
	}
}