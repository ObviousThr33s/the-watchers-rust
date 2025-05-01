
use angle_sc::{trig, Radians};

use crate::{game::{entity::player::Player, group::Group}, gfx::portal::Portal};

pub struct RayLoop {
	portal:Portal
}

impl RayLoop {
	pub fn new(){
		
	}

	pub fn cast_ray(player:Player, entities:&mut Group, max_wall_height:f64) -> Vec<(f64,f64)> {
		let mut dist = 0.0;
		let mut wall_height = 0.0;

		let entities_ = entities.entities.clone();

		let mut positions:Vec<(f64,f64)> = Vec::new(); 

		if let Some(entity) = entities.entities.get_mut("Player"){
			let mut x = entity.x as f64;
			let mut y = entity.y as f64;

			let dy = trig::sine(Radians(player.direction.0*0.174533));
			let dx = trig::cosine(Radians(player.direction.0*0.174533),dy);
			
			
			for en in entities_ {
				if en.1.id.contains("Player"){}
				else{
					let mut i = 0;
					while i < 1{
						x += dx.0*0.01;
						y += dy.0*0.01;
						
						dist = ((x - (entity.x as f64)).powf(2.0) + (y-(entity.y as f64)).powf(2.0)).sqrt();
						wall_height = max_wall_height/dist;
						
						positions.push((dist,wall_height));
		
						i += 1;
					}
				}
			}
			positions
		}else{
			positions.push((-1.0,-1.0));
			positions
		}
		
	}
}