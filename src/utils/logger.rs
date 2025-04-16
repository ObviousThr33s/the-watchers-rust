
use super::time::Time;

pub struct Logger{
	log_file: Vec<String>,
	start_time: Time
}

impl Logger {
	pub fn new(start_time:Time) -> Self{
		Logger { 
			log_file: Vec::new(),
			start_time: start_time
		}
	}

	pub fn log(&mut self, message:&str, tick:i64){
		

		self.log_file.insert(0, String::from(message));
		//let t:f64 = self.start_time.elapsed().unwrap().as_secs_f64();
		let s:String = self.start_time.to_string();
		print!("[{}]<{}> {}\n", s, tick, message);
	}
}