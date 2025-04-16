use std::{arch, os::windows::thread, sync::Arc};

use super::logger;

pub struct ElementQueue{
	e_q:Arc<Vec<i64>>,
	id_list: Vec<i64>,
	id_counter: i64
}


impl Clone for ElementQueue {
	fn clone(&self) -> Self {
		panic!("Cannot be cloned")
	}
}

impl ElementQueue {
	pub fn new(mut self, size:i64, mut logger:logger::Logger, tick:i64) -> Self{
		logger.log("Element Queue initiallizing...", tick);
		
		for i in 0..size {
			self.id_list.push(i);
		}

		ElementQueue { 
			id_list:Vec::new(),
			e_q:Arc::new(self.id_list),
			id_counter:0
		}

	}

	pub fn addToQueue(&mut self, e:element::Element) -> i64{
		let _ = e;
		self.id_queue.add_to_array(e);
		self.id_queue[id_counter];
		id_counter += 1;
		id_counter
	}

}
