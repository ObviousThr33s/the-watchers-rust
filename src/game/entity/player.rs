pub(crate) use angle_sc::{self, Degrees};


use super::{Entity, Priority};

pub struct Player {
	pub player:Entity,
	pub heading:Degrees,
	pub direction:Direction
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
	NORTH,
	SOUTH,
	EAST,
	WEST
}

impl Clone for Player {
	fn clone(&self) -> Self {
		Self { 
			player: self.player.clone(),
			heading:Degrees(0.0f64),
			direction:self.direction.clone()
		}
	}
}

impl Player {

	pub fn new() -> Self {
		Self {
			player:Entity {
				x: 2,
				y: 2,
				priority: Priority::MED,
				self_: '^', 
				id: "Player".to_owned() 
			},
			heading: angle_sc::Degrees(0.0),
			direction:Direction::NORTH
		}
	}

	pub fn add_direction(&mut self, amnt:f64){
		let old_dir = self.heading.0;
		self.heading = Degrees(old_dir) - Degrees(amnt);
	}

	pub fn sub_direction(&mut self, amnt:f64){
		let old_dir = self.heading.0;
		self.heading = Degrees(old_dir) + Degrees(amnt);
	}

	pub fn poll_move_forewards(&mut self){
		match self.player.self_ {
			'<' => self.player.move_left(),
			'>' => self.player.move_right(),
			'^' => self.player.move_up(),
			'v' => self.player.move_down(),
			 _  => ()
		}
	}

	pub fn poll_move_backwards(&mut self){
		match self.player.self_ {
			'>' => self.player.move_left(),
			'<' => self.player.move_right(),
			'v' => self.player.move_up(),
			'^' => self.player.move_down(),
			 _  => ()
		}
	}


	pub fn move_left(&mut self){
		self.add_direction(90.0);
		self.poll_heading();
		self.player.move_left();
		
	}

	pub fn move_right(&mut self){
		self.sub_direction(90.0);
		self.poll_heading();
		self.player.move_right();
	}

	fn poll_heading(&mut self) {
		// Normalize angle to 0-360 range
		let mut angle = self.heading.0 % 360.0;
		if angle < 0.0 {
			angle += 360.0;
		}
		
		// Define direction based on angle
		let (character, direction) = match angle {
			a if (0.0..=45.0).contains(&a) || (315.0..=360.0).contains(&a) => 
				('^', Direction::NORTH),
			a if (45.0..=135.0).contains(&a) => 
				('>', Direction::EAST),
			a if (135.0..=225.0).contains(&a) => 
				('v', Direction::SOUTH),
			_ => 
				('<', Direction::WEST),
		};
		
		self.player.self_ = character;
		self.direction = direction;
	}
}
