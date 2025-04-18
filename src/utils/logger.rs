
use std::path::{Path, PathBuf};

use crate::looper;

use super::{file_io::{FileData, FileOperations, FileOutStream}, time::Time};

pub struct Logger {
	log_file: Vec<String>,
	start_time: Time,
	pub tick:i128,
	f_io:FileOutStream
}

impl Clone for Logger {
	fn clone(&self) -> Self {
		Self {	log_file: self.log_file.clone(), 
				start_time: self.start_time.clone(),
				tick: self.tick.clone(),
				f_io: self.f_io.clone(),
		}
	}
}

impl Logger {
	pub fn new(start_time:Time) -> Self{
		Logger { 
			log_file: Vec::new(),
			start_time: start_time,
			tick:0,
			f_io:FileOutStream::new()
		}
	}

	pub fn log(&mut self, message:&str){
		
		let s0:String = self.start_time.to_string();
		self.tick += 1;

		let s1:String = format!("[{}]<{}> {}\n", s0,self.tick, message);
		

		self.log_file.insert(0, s1);
		//let t:f64 = self.start_time.elapsed().unwrap().as_secs_f64();
		//print!("[{}] {}\n", s, message);
	}

	pub fn get_latest_log(self) -> String{
		let log_entry: String = self.log_file.get(0)
									.cloned()
									.unwrap_or_else(|| 
										String::from("No logs available"));
		log_entry
	}

	pub fn save_log(&mut self){
		let f:FileData = FileData {
			file_path:PathBuf::from(String::from(r".\src\res\logs\log.txt")),
			id:0
		};

		self.f_io.add_file(f.clone());
		self.f_io.clone().write(f.clone(), self.log_file.concat());
	}
}