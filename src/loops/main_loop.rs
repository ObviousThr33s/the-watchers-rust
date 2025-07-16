
use std::time::Duration;

use ratatui::DefaultTerminal;
use tokio::time::sleep;
use crate::game::Game;
use crate::gfx::portal::raster::Raster;
use crate::gfx::portal::Portal;
use crate::gfx::render;
use crate::input::handle_events;
use crate::utils::{logger::Logger, time::Time};

use super::player_loop::PlayerLoop;

//See new() to update version
pub struct MainLoop{
	pub start: Time,
	pub logger:Logger,
	pub portal:Portal,
	pub game:Game,

	tick:usize,

	state:GameStates,
	terminal:DefaultTerminal,
	_output:String,
raster: Raster
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
			portal:Portal::new(),
			game:Game::new(),
			raster:Raster::new(),

			tick:0,

			//Set game version here
			logger: Logger::new(start_time, "0.4.0".to_string()),
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
			sleep(Duration::from_micros(10)).await;
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

		self.state = GameStates::Render;
		self.game.init(&mut self.logger);

		//each method requires a call back to the state loop
		self.state_loop().await.await;
	}


	//running section of the main loop
	//always a work in progress
	pub async fn run(&mut self){
		//event key which sends signals for game state and player movement
			
		let (new_state, player_input) = 
			handle_events(&mut self.terminal, &mut self.logger);

		//self.logger.log(format!("{:?}", player_input).as_str());

		if new_state == GameStates::Exit {	
			self.exit();
		}
		let (mut art, mut prompt):(String,String) = (String::new(), String::new());
		
		PlayerLoop::player_move(
			&mut self.game.player,
			player_input,
			&mut self.logger,
		);

		self.game.field.set_entity(self.game.player.player.clone());
		self.game.update(&mut art, &mut prompt, self.tick, &mut self.logger);
	

		self.portal.set_portal(art, prompt);

		self.tick += 1;

		self.logger.log(&format!("Tick: {}", self.tick));

		self.state = GameStates::Render;
		
		self.render().await;
	
	}

	pub async fn render(&mut self) {
		// Get terminal size
		let (w, h) = (self.terminal.size().unwrap().width, 
					  self.terminal.size().unwrap().height);

		self.logger.log(&format!("Size:{}x{}", w, h));


		//self.portal.build_screen(self.terminal.size().unwrap().height, self.terminal.size().unwrap().width);
		let fov = std::f32::consts::PI / 3.0;

		self.update_raster_walls();

		render(&mut self.terminal, 
			&self.logger,
			&self.game.field,
			&self.raster.to_2d5_view(
				&self.game.field, 
				self.game.player.player.get_position().0 as f32, 
				self.game.player.player.get_position().1 as f32, 
				self.game.player.heading.0 as f32 * std::f32::consts::PI / 180.0, // Convert degrees to radians
				fov,
				w as usize,
				h as usize // Half the terminal height for the 3D view
			),
			).await;

		self.state = GameStates::Run;
		self.state_loop().await.await;
	}

	fn update_raster_walls(&mut self) {
		// Clear existing walls
		self.raster.clear();
		
		// Add walls from field entities
		for (_, entity) in &self.game.field.entities {
			// Check if this entity should be treated as a wall
			// For example, entities with '#' character or specific type
			if entity.self_ == 'F' {
				// Add the entity position to the raster as a wall
				self.raster.add_point(
					entity.x,
					entity.y,
					"#".to_string()
				);
			}
		}
	}

	pub fn exit(&mut self) {
		//force halt to save files
		tokio::task::block_in_place(|| {
			self.terminal.flush().unwrap();
			
			self.terminal.clear().unwrap();
			println!("Saving log...");
	
			self.logger.save_log();
		});
		println!("Saved log.");
		println!("Exited.");
		//let _ = self.terminal.clear();
		//exit peaceably
		std::process::exit(0x0);
	}
}