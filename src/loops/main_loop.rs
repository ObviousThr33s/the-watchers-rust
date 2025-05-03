use ratatui::DefaultTerminal;
use crate::game::entity::player::{self, Player};
use crate::game::entity::wall_type::WallType;
use crate::game::spaces::field::{self, Field};
use crate::gfx::portal::raster::{self, Raster};
use crate::gfx::portal::Portal;
use crate::gfx::render;
use crate::input::handle_events;
use crate::utils::{logger::Logger, time::Time};

use super::player_loop::PlayerLoop;
use crate::game::entity::{Entity, Priority};

//See new() to update version
pub struct MainLoop{
	pub start: Time,
	pub logger:Logger,
	pub field:Field,
	pub portal:Portal,
	pub raster:Raster,

	_render_tick:usize,

	pub player:Player, //at some point this could be a hash table for many players/angle entities

	state:GameStates,
	terminal:DefaultTerminal,
	_output:String
}

//state loops definition
#[derive(PartialEq)]
pub enum GameStates{
	Init = 0,
	Run  = 1,
	Render = 2,
	Exit = 3,
}


impl MainLoop {
	pub fn new(start_time:Time, terminal:DefaultTerminal) -> MainLoop {
		

		MainLoop { 
			state: GameStates::Init,
			start:start_time.clone(),
			field: Field::new(),
			portal:Portal::new(),
			raster:Raster::new(),


			_render_tick:0,
			player:Player::new(),

			//Set game version here
			logger: Logger::new(start_time, "0.3.0".to_string()),
			_output:String::new(),
			terminal:terminal,
		}
	}

	//state loop to allow for memory management
	//using the lifetime specifiers here to keep the
	//memory from going out of scope while other processes
	//are run
	pub async fn state_loop(&mut self) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + '_>> {
		
		Box::pin(async move {
			//sleep(Duration::from_millis(1)).await;
			match self.state {
				GameStates::Exit   => self.exit(),
				GameStates::Run    => self.run().await,
				GameStates::Render => self.render().await,
				GameStates::Init   => self.init().await,
			}
		})

		
	}

	pub async  fn init(&mut self) {
		
		self.logger.log("Initializing...");

		self.field.add_entity(self.player.player.clone());
		// Add an entity at position (0, 0)
		let static_entity = Entity::new(0, 0, '+', "wall".to_owned(), Priority::LOW);
		self.field.add_entity(static_entity);
		self.logger.log("Added static entity at position (0, 0)");
		//each method requires a call back to the state
		self.state = GameStates::Render;

		self.state_loop().await.await;
	}


	//running section of the main loop
	//always a work in progress
	pub async fn run(&mut self){
		

		loop {
			//event key which sends signals for game state and player movement
			
			let (new_state, player_input) = 
				handle_events(&mut self.terminal, &mut self.logger);

			if new_state == GameStates::Exit {	
				self.exit();
				break;
			}

			PlayerLoop::player_move(
				&mut self.player,
				player_input, 
				&mut self.logger,);
			
			self.field.set_entity(self.player.player.clone());
			
			self.logger.log(&format!("{}", self.field.to_string()));

			self.state = GameStates::Render;
			self.state_loop().await.await;	
		}

	}

	pub async fn render(&mut self) {
		// Get terminal size
		let (w, h) = (self.terminal.size().unwrap().width, 
					  self.terminal.size().unwrap().height);

		self.logger.log(&format!("Size:{}x{}", w, h));
		
		// Update the raster with walls and obstacles if needed
		self.update_raster_walls();
		
		// Calculate field of view (in radians)
		let fov = std::f32::consts::PI / 3.0; // 60 degrees
		
		// Render the 2.5D view
		render(&mut self.terminal, 
			&self.logger,
			&self.field,
			&self.raster.to_2d5_view(
				&self.field, 
				self.player.player.x as f32, 
				self.player.player.y as f32, 
				self.player.heading.0 as f32 * std::f32::consts::PI / 180.0, // Convert degrees to radians
				fov,
				w as usize,
				(h as usize) // Half the terminal height for the 3D view
			)).await;

		self.state = GameStates::Run;
		self.state_loop().await.await;
	}

	// Helper method to use Field entities to update the raster walls
	fn update_raster_walls(&mut self) {
		// Clear existing walls
		self.raster.clear();
		
		// Create boundary walls of stone (keeping this for world boundaries)

		// Add walls based on entities in the field
		for (_, entity) in &self.field.entities {
			// Skip the player entity (don't want player to be a wall)
			if entity.id == self.player.player.id {
				continue;
			}
			
			// Determine wall type based on entity properties
			let wall_type = self.determine_wall_type(entity);
			
			// Add to raster
			self.raster.add_wall_point(entity.x as u16, entity.y as u16, wall_type);
		}
	}

	// Helper method to determine wall type from an entity
	fn determine_wall_type(&self, entity: &Entity) -> WallType {
		// Determine wall type based on entity character or ID
		match entity.self_ {
			'#' => WallType::Stone,
			'+' => WallType::Wood,
			'M' => WallType::Metal,
			'G' => WallType::Glass,
			'B' => WallType::Brick,
			// For entities with other characters, use their ID to determine type
			_ => match entity.id.as_str() {
				id if id.contains("wall") => WallType::Stone,
				id if id.contains("wood") => WallType::Wood,
				id if id.contains("metal") => WallType::Metal,
				id if id.contains("glass") => WallType::Glass, 
				id if id.contains("brick") => WallType::Brick,
				// Default to a custom wall with the entity's character
				_ => WallType::Custom(entity.self_),
			},
		}
	}

	pub fn exit(&mut self) {
		
		println!("Saving log...");
		//force halt to save files
		tokio::task::block_in_place(|| {
			self.logger.save_log();
		});

		//let _ = self.terminal.clear();
		//exit peaceably
		std::process::exit(0x0);
	}
}