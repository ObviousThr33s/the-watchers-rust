
use std::collections::HashMap;
use std::time::Duration;

use rand::rngs::ThreadRng;
use ratatui::DefaultTerminal;
use tokio::time::sleep;
use crate::game::entity::Entity;
use crate::game::group::Group;
use crate::gfx::render;
use crate::input::handle_events;
use crate::utils::{logger::Logger, time::Time};

use super::loops::main_loop::MainLoop;


//declare the main loop as a struct
//given a start time for the logger and the logger itself
//given a game state to track which fn to run
//each function activates the state loop
//terminal and debug output string
pub struct Looper{
	pub start: Time,
	pub logger:Logger,
	pub entities:Group,

	state:GameStates,
	tick:usize,
	
	terminal:DefaultTerminal,
	_output:String
}

#[derive(PartialEq)]
pub enum GameStates{
	Init = 0,
	Run  = 1,
	Exit = 2
}

impl Looper {
	pub fn new(start_time:Time, terminal:DefaultTerminal) -> Looper {
		

		Looper { 
			tick: 0, 
			state: GameStates::Init,
			start:start_time.clone(),
			entities:Group { entities: HashMap::new() },
			//Set game version here

			logger: Logger::new(start_time, "0.2.2".to_string()),
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
			//sleep(Duration::from_secs(1)).await;
			match self.state {
				GameStates::Exit   => self.exit().await,
				GameStates::Run    => self.run().await,
				GameStates::Init   => self.init().await,
			}
		})

		
	}

	pub async fn init(&mut self) {
		
		self.logger.log("Initializing...");
		
		self.tick += 1;

		self.logger.log("Initializing done");
		self.state = GameStates::Run;

		self.state_loop().await.await;
	}

	

	//running section of the main loop
	//always a work in progress
	pub async fn run(&mut self){

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
			x: 15,
			y: 15,
			self_: 'O', 
			id: "Obol".to_owned() 
		});


		

		loop {

			let (new_state, player_input) = 
				handle_events(&mut self.terminal, &mut self.logger);
			
			if new_state == GameStates::Exit {
				
				self.logger.log("Exited");
					render(
						&mut self.terminal, 
						self.logger.clone(), 
						Group::new()
					).await;	
				std::thread::sleep(Duration::from_secs(3));
				let _ = self.terminal.clear();
				break;
			}else{
				self.state = new_state;
			}

			let entity = 
				MainLoop::main_loop(self.entities.clone(), player_input);

			self.entities = entity.await.clone();

			self.logger.log(&self.entities.get_entity("Player".to_owned()).unwrap().to_string());

			render(
				&mut self.terminal,
				
				self.logger.clone(),
				
				self.entities.clone()
			).await;

			
		}


		self.state = GameStates::Exit;
		self.state_loop().await.await;
	
	}

	pub async fn exit(&mut self) {
		
		println!("Saving log...");
		sleep(Duration::from_secs(1)).await;

		tokio::task::block_in_place(|| {
			self.logger.save_log();
		});

		//let _ = self.terminal.clear();
		std::process::exit(0x0);
	}
}