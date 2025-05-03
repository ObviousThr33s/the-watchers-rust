use crate::{game::{entity::{Entity, Priority}, spaces::field::Field}, gfx::{charset::CHARSETS, screen::Screen}};

pub struct Render{
	render: Screen,
	charset: CHARSETS,
}



impl Clone for Render {
	fn clone(&self) -> Self { 
		let render:Render = Render { render: self.render.clone(), charset: self.charset };
		render
	} 
}

impl ToString for Render {
	fn to_string(&self) -> String {
		self.render.screen.iter().collect()
	}
}

impl Render {

	pub fn init(width:usize, height:usize, charset:CHARSETS) -> Self{
		let scr:Screen = Screen::new(width, height);
		let lamp_ = Render {			
			render: scr,
			charset: charset,
		};
		
		
		
		lamp_
	}
	
	pub fn rasterize(&mut self, field:&Field){

		//first clear screen buffer
		self.render.screen.clear();

		//get two clones of the entities
		let entities1 = field.entities.clone();
		let entities2 = field.entities.clone();
		
		//create an empty raster
		let mut raster:Vec<Entity> = Vec::new();

		//super tuple for all the data from the entities
		let mut x:Vec<usize> = Vec::new();
		let mut y:Vec<usize> = Vec::new();
		let mut p:Vec<Priority> = Vec::new();
		let mut id:Vec<String> = Vec::new();

		//fill the super tuple
		for e in entities1.iter(){
			let (x_, y_, p_) = 
							(e.1.x.clone(), e.1.y.clone(), e.1.priority.clone());
			x.push(x_);
			y.push(y_);
			p.push(p_);
			id.push(e.0.clone());
		}

		//if any element in has the same x,y as another, only put the element with
		//the highest priority on the board
		for (_i, e) in entities2.iter() {
			let (x_, y_, p_) = 
												(e.x, e.y, e.priority.clone());
			let mut should_add = true;

			for existing in raster.iter_mut() {
				if existing.x == x_ && existing.y == y_ {
					if p_ > existing.priority {
						if let Some(existing) = raster.iter_mut()
										.find(|existing| 
											existing.x == x_ && existing.y == y_) 
						{
							*existing = e.clone();
						}
					}
					should_add = false;
					break;
				}
			}

			if should_add {
				raster.push(e.clone());
			}
		}

		let mut flag = false;


		//push to render from pseudo raster
		for i in 0..self.render.y {
			for j in 0..self.render.x {
				for e in &raster{
					if j == e.x && i == e.y {
						self.render.screen.push(e.self_);
						flag = true;
						break;
					}
				}
				if flag == true{
					flag = false;
				}else{
					self.render.screen.push(' ');
				}
			}
			self.render.screen.push('\n');
		}
}
}
