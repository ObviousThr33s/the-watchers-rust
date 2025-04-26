
use std::clone;
use std::os::windows::thread;
use std::path::Path;
use std::time::Duration;

use ratatui::DefaultTerminal;
use tokio::time::sleep;
use crate::game::entity::{self, Entity};
use crate::game::terrain;
use crate::gfx::{self, render};
use crate::input::handle_events;
use crate::utils::{logger::Logger, time::Time};

use super::loops::main_loop::MainLoop;
use super::loops::{self};


//declare the main loop as a struct
//given a start time for the logger and the logger itself
//given a game state to track which fn to run
//each function activates the state loop
//terminal and debug output string
pub struct Looper{
	pub start: Time,
	pub logger:Logger,

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
			sleep(Duration::from_secs(1)).await;
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
		
		//create a list of entities to display and the player
		let mut entities_: Vec<Entity> = Vec::new();
		let mut player:Entity = Entity { x: 10, y: 10, self_: '@' };

		//main loop here
		loop {
			//return the player movement, needs to return signals instead
			let new_state = handle_events(&mut self.terminal, &mut self.logger, &mut player);
			
			self.state = new_state;
			self.tick += 1;

			//create a special new main loop for non systems game logic only 
			let mut ml: MainLoop = loops::main_loop::MainLoop::new();
			//let tick;
			//tick = ml.main_loop(&mut event_manager).await;
			
			self.tick += 1;
			
			//let mov_x = 1;
			//let mov_y = 1;

			//the current entities on screen
			entities_ = vec![
				player.clone(),
				Entity{
					x:0,
					y:0,
					self_:'E',
				}
			];
			
			//render objects and entities, for now, only the logger, soon the inv, and stats, 
			//as well as a map and maybe a compas bar.
			entities_ = render(
				&mut self.terminal, 
				&mut self.logger, 
				&mut entities_
			).await;

			self.logger.log(&format!("({},{})", entities_[0].x, entities_[0].y));
			self.logger.log("Rendering finished");

			self.tick += 1;

			// Break the loop when transitioning to the Exit state
			if self.state == GameStates::Exit {
				break;
			}
		}
		std::thread::sleep(Duration::from_secs(3));
		self.logger.log("Exiting");
		self.state = GameStates::Exit;
		self.state_loop().await.await;
	}

	pub async fn exit(&mut self) {
		
		
		
		//sleep(Duration::from_millis(1000)).await;
		
		println!("Saving log...");
		sleep(Duration::from_secs(1)).await;

		tokio::task::block_in_place(|| {
			self.logger.save_log();
		});

		//let _ = self.terminal.clear();
		std::process::exit(0x0);
	}
}