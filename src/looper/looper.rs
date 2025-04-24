
use std::time::Duration;

use ratatui::DefaultTerminal;
use crate::gfx::{self, render};
use crate::input::handle_events;
use crate::utils::{logger::Logger, time::Time};

pub struct Looper{
	pub start: Time,
	pub logger:Logger,

	state:GameStates,
	tick:i64,
	
	terminal:DefaultTerminal,
	_output:String
}

#[derive(PartialEq)]
pub enum GameStates{
	Init = 0,
	Run  = 1,
	Render = 2,
	Exit = 3
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
				GameStates::Render => gfx::render(&mut self.terminal, &mut self.logger).await, //may panic?
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
		
		let _tick_max = 20;//10f64.powf(127.0);
		
		//element que
		//gen world closure
		//gen one sub group
		//transform the sub groups

		loop{
			self.tick += 1;
			self.logger.log("Running");
			
			let _ = render(&mut self.terminal, &mut self.logger).await;
			
			
			if handle_events(&mut self.terminal, &mut self.logger).unwrap_or(false) {
				break
			}
		}

		self.state = GameStates::Exit;
		self.state_loop().await;
	}

	pub async fn exit(&mut self) {

		self.logger.log("Exiting");
		
		let _ = render(&mut self.terminal, &mut self.logger).await;
		std::thread::sleep(Duration::from_secs(1));

		gfx::clear(&mut self.terminal);

		std::thread::sleep(Duration::from_secs(1));
		self.logger.log("Saving log...");
		let _ = self.logger.save_log().await;

		std::thread::sleep(Duration::from_secs(1));
		
		std::process::exit(0x0);
	}

	pub async fn draw_menu(mut self){
		
		self.state = GameStates::Exit;
		self.state_loop().await;
	}
}