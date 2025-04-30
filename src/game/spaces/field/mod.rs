
use crate::game::{entity::{player::Player, Entity, Priority}, group::Group};

pub struct Field{
	pub entities:Group,
	pub player:Player
}

impl Clone for Field {
	fn clone(&self) -> Self {
		Self { 
			entities: self.entities.clone(), 
			player:Player::new(0.0f64)
		}
	}
}

impl ToString for Field {
	fn to_string(&self) -> String {
		let mut s_:Vec<String> = Vec::new();
		s_.push(String::from("\n"));

		for i in self.entities.entities.clone() {
			s_.push(format!("\t{}\n", i.0));
		}

		let s = s_.concat();
		s
	}
}

impl Field{

	pub fn new() -> Self{
		Field { entities: Group::new(), player:Player::new(0.0f64) }
	}

	pub fn get_entities(self) -> Group {
		self.entities.clone()
	}

	pub fn gen_entities(&self, mut entities:Group) -> Group{
		entities.entities.insert("Player".to_owned(), 
		self.player.player.clone());
	
		entities.entities.insert("Entity".to_owned(), 
		Entity {
			x: 10,
			y: 10,
			priority: Priority::LOW,
			self_: 'E', 
			id: "Entity".to_owned() 
		});
	
		entities.entities.insert("Obol".to_owned(), 
		Entity {
			x: 0,
			y: 0,
			priority:Priority::HIG,
			self_: 'O', 
			id: "Obol".to_owned() 
		});

		
	
		entities
	}
	
}

