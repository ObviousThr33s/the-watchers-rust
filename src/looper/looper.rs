
use std::sync::Arc;
use std::time::Duration;

use rand::Rng;
use ratatui::{DefaultTerminal, Terminal};
use tokio::time::sleep;
use crate::game::entity::Entity;
use crate::game::group::{self, Group};
use crate::gfx::render;
use crate::input::{handle_events, PlayerMove};
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


		
		//create a list of entities to display and the player
		
		
		
		let player:Entity = Entity { x: 40, y: 5, self_: '@', id:"Player".to_owned() };
		let foo:Entity    = Entity { 
			x: 10,
			y: 10, 
			self_: 'E' ,
			id:"Entity 1".to_owned()
		};
		

		let mut current_world:Group = Group::new(vec![player,foo]);

		let mut r = rand::rng();
		let mut ml: MainLoop = loops::main_loop::MainLoop::new();

		pub fn change_entity(){
			 = Entity { x: r.random_range(0..100), y: r.random_range(0..100), self_: 'e' };
		}

		//main loop here
		loop {
			let _ = rand::rng().reseed();
			//return the player movement, needs to return signals instead
			let (new_state, player_move) = handle_events(&mut self.terminal, &mut self.logger, &mut entities_);
			
			if new_state == GameStates::Exit {
				
				self.logger.log("Exited");
					render(
						&mut self.terminal, 
						self.logger.clone(), 
						&mut entities_
					).await;	
				std::thread::sleep(Duration::from_secs(3));
				self.terminal.clear();
				break;
			}else{
				self.state = new_state;
			}


			if player_move == PlayerMove::UP {
				entities_[0].move_up();
			}
			if player_move == PlayerMove::DOWN {
				entities_[0].move_down();
			}
			if player_move == PlayerMove::LEFT {
				entities_[0].move_left();
			}
			if player_move == PlayerMove::RIGHT {
				entities_[0].move_right();

			}

			
		

			self.tick += 1;

			//create a special new main loop for non systems game logic only 
			//let tick;
			
			self.tick += 1;
			
			//let mov_x = 1;
			//let mov_y = 1;

			//the current entities on screen
			
			
			//render objects and entities, for now, only the logger, soon the inv, and stats, 
			//as well as a map and maybe a compas bar.
			(entities_) = render(
				&mut self.terminal, 
				self.logger.clone(), 
				&mut entities_
			).await;

			//(entities_, self.tick) = ml.main_loop(&mut self.logger, entities_,self.tick, r.clone()).await;
			
			self.logger.log(&entities_[0].to_string());
			

			self.tick += 1;

			// Break the loop when transitioning to the Exit state
			
		}


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