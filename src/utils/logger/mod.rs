
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;


use super::time::Time;


pub struct Logger {
	log_stream: Vec<String>,
	start_time: Time,
	pub tick:usize,
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
	/// Creates a new `Logger` instance.
	///
	/// # Parameters
	/// - `start_time`: The starting time used to initialize the logger.
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
		self.tick += 1;

		let s1: String = format!("[{}]<{}> {}\n", s0, self.tick, message);
		self.log_stream.push(s1);
	}

	pub fn get_log(mut self, lines: usize) -> Vec<String> {
		let mut stream: Vec<String> = self.log_stream.to_owned();
		
		if lines >= stream.len() {
			return vec!["Not enough log lines input...".to_string()];
		} 
		
		stream.reverse();
		let s:Vec<String> = stream.to_owned();
		s
	}

	pub fn get_version(self) -> String {
		self.vers
	}

	pub fn save_log(&mut self) {
		let dir = "./res/";
		let file_name = "log.txt";

		let mut file_path = PathBuf::from(dir);
		file_path.push(file_name);

		// Ensure the directory exists
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