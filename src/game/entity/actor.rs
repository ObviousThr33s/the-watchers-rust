use std::{fs::File, io::Read, path::{Path, PathBuf}};


pub struct Actor{
	pub(crate) name: String,
	pub(crate) health: i32,
	pub(crate) attack_power: i32,
	pub art:String,
	pub prompt:String
}

impl Clone for Actor {
	fn clone(&self) -> Self {
		Self {	
			name: self.name.clone(),
			health: self.health, 
			attack_power: self.attack_power, 
			art: self.art.clone(), 
			prompt: self.prompt.clone() 
		}
	}
}

impl ActorData for Actor {
	fn name(&self) -> &str {
		&self.name
	}

	fn name_mut(&mut self) -> &mut String {
		&mut self.name
	}

	fn health(&self) -> i32 {
		self.health
	}

	fn health_mut(&mut self) -> &mut i32 {
		&mut self.health
	}

	fn attack_power(&self) -> i32 {
		self.attack_power
	}

	fn attack_power_mut(&mut self) -> &mut i32 {
		&mut self.attack_power
	}

	fn art(&self) -> &str {
		&self.art
	}

	fn art_mut(&mut self) -> &mut String {
		&mut self.art
	}

	fn new(name: String, health: i32, attack_power: i32) -> Self {
		Actor { name, health, attack_power, art:String::new(), prompt:String::new() }
	}

	fn set_name(&mut self, name:String) {
		self.name = name
	}
}

pub trait ActorData {
	fn name(&self) -> &str;
	fn name_mut(&mut self) -> &mut String;
	fn health(&self) -> i32;
	fn health_mut(&mut self) -> &mut i32;
	fn attack_power(&self) -> i32;
	fn attack_power_mut(&mut self) -> &mut i32;
	fn art(&self) -> &str;
	fn art_mut(&mut self) -> &mut String;

	fn new(name: String, health: i32, attack_power: i32) -> Self;

	fn set_name(&mut self, name: String);

	fn set_art_from_file(&mut self, name:String) {
		let cwd = std::env::current_dir().expect("Failed to get current directory");
		let path = &format!("{}/res/entities/{}/art.txt", cwd.display(), name);
		let art_file = PathBuf::from(path);
		let mut file = File::open(&art_file).unwrap_or_else(|_| panic!("Error loading art file for: {} at {}", name, art_file.display()));
		let mut buf:String = String::new();
		file.read_to_string(&mut buf).expect("Error reading art file");
		*self.art_mut() = buf;
	}

	fn set_stats_from_file(&mut self, file_path: &str) {
		// Load the stats from the file
		let stats = std::fs::read_to_string(Path::new(file_path)).unwrap_or_else(|_| "Error loading stats".to_string());
		let parts: Vec<&str> = stats.split(',').collect();
		if parts.len() == 3 {
			*self.name_mut() = parts[0].to_string();
			*self.health_mut() = parts[1].parse().unwrap_or(0);
			*self.attack_power_mut() = parts[2].parse().unwrap_or(0);
		}
	}
	
	fn get_art(&self) -> &str {
		self.art()
	}

	fn get_stats(&self) -> (String, i32, i32) {
		(self.name().to_string(), self.health(), self.attack_power())
	}
}