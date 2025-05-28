use std::collections::HashSet;

use crate::game::entity;

#[derive(Hash, Eq, PartialEq)]
pub struct Hap {
	pub id: String,
	pub priority: entity::Priority,
	pub self_: (),
}

pub struct Haps {
	ev: HashSet<Hap>
}

impl Haps {
	pub fn new() -> Self {
		Self{
			ev: HashSet::new()
		}
	}
	
	pub fn add_event(&mut self, event:Hap) {
		self.ev.insert(event);
	}

	pub fn execute(&self) -> () {
		for e in &self.ev {
			e.self_
		}
	}
}