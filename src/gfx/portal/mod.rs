
use std::char;

use pixel::Pixel;
use raster::Raster;

use crate::{game::{entity::player::Player, group::Group}, loops::ray_loop::RayLoop, utils::logger::Logger};

use super::screen::{self, Screen};

pub mod raster;
pub mod pixel;

pub struct Portal {
	pub screen:Screen,
	pub raster:Raster,
	pub rays:Vec<(f64,f64)>
}

impl Portal {

	pub fn new() -> Self {
		Self { screen: Screen::new(0, 0), raster: Raster::new(Vec::new()), rays:vec![] }
	}

	pub fn create_raster(&mut self, player:Player, mut entity:Group, w:i32, h:i32) -> Vec<(f64, f64)>{
		let mut ray:Vec<(f64, f64)> = vec![];
		self.raster.clear();
		
		for _i in 0..w*h{
			ray.append(&mut RayLoop::cast_ray(player.clone(), &mut entity, 5.0));
		}
		ray
	}

	pub fn make_wall_slice(wall_height:f64, dist:f64) -> (usize, f64){
		let wh = wall_height.floor() as usize;
		let d = (dist*10.0).ceil();

		(wh, d)
	}

	pub fn fill_raster(&mut self, width:u16, height:u16){
		//self.raster.clear();
			
		for i in self.rays.clone(){
			let wall = Portal::make_wall_slice(i.0, i.1);

			for x in 0..height{
				for y in 0..width {
					if (y as usize) < wall.0 {
						self.raster.push(Pixel::new(x, y, "X".to_owned()));
					}
					if (y as usize) < wall.0 || (y as usize) > wall.0{
						self.raster.push(Pixel::new(x, y, " ".to_owned()));
					}
				}
			}
		}
		Logger::save_log_sp("res", "ray_act", self.raster.clone().to_string(width,height));

	}
}