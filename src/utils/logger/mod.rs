
use std::path::PathBuf;
use super::file_io::{file_outstream, FileOperations};

use super::{file_io::{file_data::FileData, file_outstream::FileOutStream}, time::Time};

pub struct Logger {
	log_stream: Vec<String>,
	start_time: Time,
	pub tick:usize,
	f_io:FileOutStream
}

impl Clone for Logger {
	fn clone(&self) -> Self {
		Self {	log_stream: self.log_stream.clone(), 
				start_time: self.start_time.clone(),
				tick: self.tick.clone(),
				f_io: self.f_io.clone(),
		}
	}
}

impl Logger {
	/// Creates a new `Logger` instance.
	///
	/// # Parameters
	/// - `start_time`: The starting time used to initialize the logger.
	pub fn new(start_time:Time) -> Self {
		
		Logger { 
			log_stream: Vec::new(),
			start_time: start_time,
			tick: 0,
			f_io:file_outstream::FileOutStream::new(),
		}
	}

	pub fn log(&mut self, message: &str) {
		let s0: String = self.start_time.to_string();
		self.tick += 1;

		let s1: String = format!("[{}]<{}> {}\n", s0, self.tick, message);
		self.log_stream.push(s1);
	}

	pub fn get_log(mut self, lines: usize) -> Vec<String> {
		let mut split_stream = self.log_stream.split_off(lines);
		split_stream.reverse();
		let stream: Vec<String> = split_stream.to_owned();
		stream
	}

	pub async  fn save_log(&mut self) {
		self.f_io.add_file(String::from("\\res\\logs\\"),String::from("log"));
	}
}