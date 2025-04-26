
use std::env;
use std::fs::File;
use std::io::Write;
use std::os::windows::thread;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::thread::{sleep, Thread};
use std::time::Duration;


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
		let mut split_stream = self.log_stream.split_off(lines);
		split_stream.reverse();
		let stream: Vec<String> = split_stream.to_owned();
		stream.clone()
	}

	pub fn get_version(self) -> String {
		self.vers
	}

	#[allow(unused)]
	pub async fn save_log(&mut self) {
		let dir = "C:\\Users\\kfman\\Desktop\\Portfolio\\Software\\Rust\\the-watchers-rust";
		let file_name = "log.txt";

		let mut file_path = PathBuf::from(dir);
		file_path.push(file_name);

		
	}
}