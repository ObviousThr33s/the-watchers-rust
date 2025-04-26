
use std::time::Duration;

use ratatui::DefaultTerminal;
use crate::game::entity::{self, Entity};
use crate::gfx::{self, render};
use crate::input::handle_events;
use crate::utils::{logger::Logger, time::Time};

use super::loops::main_loop::MainLoop;
use super::loops::{self};

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
		
			logger: Logger::new(start_time, "0.1.9".to_string()),
			_output:String::new(),
			terminal:terminal,
		}
	}

	pub fn state_loop(&mut self) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + '_>> {

		Box::pin(async move {
			match self.state {
				GameStates::Exit   => self.exit().await,
				GameStates::Run    => self.run().await,
				GameStates::Init   => self.init().await,
			}
		})

		
	}

	pub async fn init(&mut self) {
		
		self.logger.log("Initializing...");
		
		//neeed multithreading here, am lazy
		self.tick += 1;

		self.logger.log("Initializing done");
		self.state = GameStates::Run;



		self.state_loop().await;
	}

	pub async fn run(&mut self){
		
		let mut entities_: Vec<Entity> = Vec::new();
		let mut player:Entity = Entity { x: 10, y: 10, self_: '@' };


		loop {
			let e:Entity = handle_events(&mut self.terminal, &mut self.logger, &mut player).clone();
			self.tick += 1;

			let mut ml: MainLoop = loops::main_loop::MainLoop::new();
			//let tick;
			//tick = ml.main_loop(&mut event_manager).await;
			
			self.tick += 1;
			
			//let mov_x = 1;
			//let mov_y = 1;

			entities_ = vec![
				e
			];

			entities_ = render(&mut self.terminal, &mut self.logger, &mut vec![
				entities_[0].clone()
			]).await;

			self.logger.log(&format!("({},{})", entities_[0].x, entities_[0].y));
			self.logger.log("Rendering finished");

			self.tick += 1;

			// Break the loop when transitioning to the Exit state
			if self.state == GameStates::Exit {
				break;
			}
		}

		self.state = GameStates::Exit;
		self.state_loop().await;
	}

	pub async fn exit(&mut self) {
		std::thread::sleep(Duration::from_secs(3));

		self.logger.log("Exiting");
		
		self.logger.save_log().await;
		
		println!("Saving log...");
		
		
		
		std::process::exit(0x0);
	}
}