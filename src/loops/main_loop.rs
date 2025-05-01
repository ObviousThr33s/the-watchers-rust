

use ratatui::DefaultTerminal;
use crate::game::spaces::field::Field;
use crate::gfx::portal::Portal;
use crate::gfx::render;
use crate::input::handle_events;
use crate::utils::{logger::Logger, time::Time};

use super::player_loop::PlayerLoop;
use super::world_loop;

//See new() to update version
pub struct MainLoop{
	pub start: Time,
	pub logger:Logger,
	pub field:Field,
	pub portal:Portal,

	_render_tick:usize,

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


			_render_tick:0,

			//Set game version here
			logger: Logger::new(start_time, "0.2.5".to_string()),
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

		//init the world loop
		//currently there is no loop update
		world_loop::WolrdLoop::init(&mut self.field);
		

		//each method requires a call back to the state
		self.state = GameStates::Run;

		self.state_loop().await.await;
	}


	//running section of the main loop
	//always a work in progress
	pub async fn run(&mut self){
		

		loop {
			//event key which sends signals for game state and player movement
			let (new_state, player_input) = 
				handle_events(&mut self.terminal, &mut self.logger).await;

			if new_state == GameStates::Exit {	
				self.exit();
				break;
			}

			PlayerLoop::player_move(
				&mut self.field.entities,
				&mut self.field.player,
				player_input, 
				&mut self.logger,
			);

			self.state = GameStates::Render;
			self.state_loop().await.await;	
		}

	}

	pub async fn render(&mut self){
		//get terminal size at terminal resize (hopefully)
		let (w,h) = (self.terminal.size().unwrap().width, 
								self.terminal.size().unwrap().height);

		self.logger.log(&format!("Size:{}x{}", w, h));		
	
		self.portal.fill_raster(w,h);
		


		//render the frame in time with the event key
		tokio::task::block_in_place(|| {
			render(&mut self.terminal, 
				self.logger.clone(), 
				self.field.entities.clone(),
				self.portal.raster.to_string(w, h).clone());
			});

		self.state = GameStates::Run;
		self.state_loop().await.await;

		
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