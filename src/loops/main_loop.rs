
use std::collections::HashMap;
use std::time::Duration;

use ratatui::DefaultTerminal;
use tokio::time::sleep;
use crate::game::entity::Entity;
use crate::game::group::Group;
use crate::gfx::render;
use crate::input::handle_events;
use crate::utils::{logger::Logger, time::Time};

use super::player_loop::PlayerLoop;


//declare the main loop as a struct
//given a start time for the logger and the logger itself
//given a game state to track which fn to run
//each function activates the state loop
//terminal and debug output string
pub struct MainLoop{
	pub start: Time,
	pub logger:Logger,
	pub entities:Group,

	state:GameStates,
	terminal:DefaultTerminal,
	_output:String
}

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
			entities: Group { entities: HashMap::new() },
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
			sleep(Duration::from_millis(1)).await;
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

		self.entities.entities.insert("Player".to_owned(), 
		Entity {
			x: 0,
			y: 0,
			self_: '@', 
			id: "Player".to_owned() 
		});

		self.entities.entities.insert("Entity".to_owned(), 
		Entity {
			x: 10,
			y: 10,
			self_: 'E', 
			id: "Entity".to_owned() 
		});

		self.entities.entities.insert("Obol".to_owned(), 
		Entity {
			x: 0,
			y: 0,
			self_: 'O', 
			id: "Obol".to_owned() 
		});



		self.logger.log("Initializing done");
		self.state = GameStates::Run;

		self.state_loop().await.await;
	}

	

	//running section of the main loop
	//always a work in progress
	pub async fn run(&mut self){

		
		loop {

			let (new_state, player_input) = 
				handle_events(&mut self.terminal, &mut self.logger).await;
			
			if new_state == GameStates::Exit {	
				self.exit();
				break;
			}

			PlayerLoop::player_move(
				&mut self.entities,
				player_input, 
				&mut self.logger,
			);
			self.state = GameStates::Render;
			self.state_loop().await.await;	
		}



	}

	pub async fn render(&mut self){

		render(&mut self.terminal, 
			self.logger.clone(), 
			self.entities.clone());

		self.state = GameStates::Run;
		self.state_loop().await.await;
	}

	pub fn exit(&mut self) {
		
		println!("Saving log...");
		
		tokio::task::block_in_place(|| {
			self.logger.save_log();
		});

		//let _ = self.terminal.clear();
		std::process::exit(0x0);
	}
}