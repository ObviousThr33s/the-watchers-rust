extern crate chrono;

use std::time::SystemTime;
use chrono::prelude::DateTime;
use chrono::Utc;

pub struct Time {
	pub start_time: SystemTime,
}

impl Clone for Time {
	fn clone(&self) -> Self {
		
		Self { 
			start_time: self.start_time,
		}
	
	}

}

impl ToString for Time {

	fn to_string(&self) -> String {

		let time = Self::get_system_time(); 

		let datetime: DateTime<Utc> = time.into();
		let out: String = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
		out
	}
}

impl Time {
	pub fn new() -> Self{
	
		Time { start_time: Self::get_system_time() }
	
	}

	pub fn get_current_time() -> SystemTime {
		let st:SystemTime = SystemTime::now();
		st.clone()
	}

	pub fn get_system_time() -> SystemTime {
		let n: SystemTime = SystemTime::now();
		n
	}

}



