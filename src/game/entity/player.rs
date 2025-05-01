
use std::{f32::consts::PI, fmt::Pointer};

use micromath::F32Ext;
use angle_sc::{self, trig::sine, Degrees, Radians};
use ratatui::symbols::bar::SEVEN_EIGHTHS;

use crate::input::PlayerMove;

use super::{Entity, Priority};

pub struct Player {
	pub player:Entity,
	pub direction:Degrees
}

impl Clone for Player {
	fn clone(&self) -> Self {
		Self { 
			player: self.player.clone(),
			direction:Degrees(0.0f64)
		}
	}
}

impl Player {

	pub fn new(direction: f64) -> Self {
		Self {
			player:Entity {
				x: 2,
				y: 2,
				priority: Priority::MED,
				self_: '@', 
				id: "Player".to_owned() 
			},
			direction: Degrees(direction),
		}
	}

	pub fn add_direction(&mut self, amnt:f64){
		let old_dir = self.direction.0;
		self.direction = Degrees(old_dir) - Degrees(amnt);
	}

	pub fn sub_direction(&mut self, amnt:f64){
		let old_dir = self.direction.0;
		self.direction = Degrees(old_dir) + Degrees(amnt);
	}

	pub fn poll_move_forewards(&mut self, entity:&mut Entity){
		match entity.self_ {
			'<' => entity.move_left(),
			'>' => entity.move_right(),
			'^' => entity.move_up(),
			'v' => entity.move_down(),
			 _  => ()
		}
	}

	pub fn poll_move_backwards(&mut self, entity:&mut Entity){
		match entity.self_ {
			'>' => entity.move_left(),
			'<' => entity.move_right(),
			'v' => entity.move_up(),
			'^' => entity.move_down(),
			 _  => ()
		}
	}


	pub fn move_left(&mut self, entity:&mut Entity){
		self.add_direction(45.0);
		self.poll_heading(self.direction, entity);
		
	}

	pub fn move_right(&mut self, entity:&mut Entity){
		self.sub_direction(45.0);
		self.poll_heading(self.direction, entity);

	}

	fn poll_heading(&mut self, deg:Degrees, entity:&mut Entity) {
		if deg.0 >= -45.0 && deg.0 <= 45.0 {
			entity.self_ = '^';
		}else if deg.0 >= 45.0 && deg.0 <= 135.0 {
			entity.self_ = '>';
		}else if deg.0 >= 135.0 && deg.0 <= 180.0 {
			entity.self_ = 'v';
		}else{
			entity.self_ = '<';
		}

		
	}
}
