use std::{fs::File, io::{BufReader, Read}, path::Path};

use super::Entity;

pub struct Actor{
	name: String,
	health: i32,
	attack_power: i32,
	pub art:String
}

impl Clone for Actor {
	fn clone(&self) -> Self {
		Self { name: self.name.clone(), health: self.health.clone(), attack_power: self.attack_power.clone(), art: self.art.clone() }
	}
}

impl Actor {
	pub fn new(name: String, health: i32, attack_power: i32) -> Self {
		Actor { name, health, attack_power, art: String::new() }
	}

	pub fn set_art_from_file(&mut self) {
		let path = &format!("./res/entities/{}/art.txt", self.name);
		let art_file = Path::new(path);
		let mut file = File::open(art_file).unwrap_or_else(|_| panic!("Error loading art file for: {}", self.name));
		let mut buf:String = String::new();
		file.read_to_string(&mut buf).expect("Error reading art file");
		self.art = buf.clone();
	}

	pub fn set_stats_from_file(&mut self, file_path: &str) {
		// Load the stats from the file
		let stats = std::fs::read_to_string(Path::new(file_path)).unwrap_or_else(|_| "Error loading stats".to_string());
		let parts: Vec<&str> = stats.split(',').collect();
		if parts.len() == 3 {
			self.name = parts[0].to_string();
			self.health = parts[1].parse().unwrap_or(0);
			self.attack_power = parts[2].parse().unwrap_or(0);
		}
	}
	pub fn get_art(&self) -> &str {
		&self.art
	}

	pub fn get_stats(&self) -> (String, i32, i32) {
		(self.name.clone(), self.health, self.attack_power)
	}

}

